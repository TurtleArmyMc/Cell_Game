use cell_game::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    ids::PlayerId,
    player_info::PlayerInfo,
    pos::{Circle, Point},
};

use crate::view_snapshot::ViewSnapshot;

/// Keeps track of previous view states so that they can be interpolated
/// between when rendering.
pub struct ViewHistory {
    curr: Option<ViewSnapshot>,
    prev: Option<ViewSnapshot>,
}

pub struct InterpolatedView<'a> {
    curr: &'a ViewSnapshot,
    prev: &'a ViewSnapshot,
    delta: f64,
}

pub enum BufferedView<'a> {
    Snapshot(&'a ViewSnapshot),
    Interpolated(InterpolatedView<'a>),
}

impl ViewHistory {
    pub fn new() -> Self {
        Self {
            curr: None,
            prev: None,
        }
    }

    pub fn update<'a, V: GameView>(&mut self, view: &'a V) {
        self.prev = self.curr.take();
        self.curr = Some(ViewSnapshot::new(view));
    }

    pub fn get_interpolated_view(&self, delta: f64) -> Option<BufferedView> {
        if let (Some(curr), Some(prev)) = (self.curr.as_ref(), self.prev.as_ref()) {
            Some(BufferedView::Interpolated(InterpolatedView {
                curr,
                prev,
                delta,
            }))
        } else {
            self.curr.as_ref().map(BufferedView::Snapshot)
        }
    }
}

impl GameView for InterpolatedView<'_> {
    fn player_cells(&self) -> impl Iterator<Item = PlayerCell> {
        self.curr.player_cells().map(|mut cell| {
            if let Some(prev_cell) = self
                .prev
                .player_cells()
                .find(|prev_cell| prev_cell.id() == cell.id())
            {
                *cell.pos_mut() = Self::lerp_point(prev_cell.pos(), cell.pos(), self.delta);
                *cell.mass_mut() = Self::lerp_f64(prev_cell.mass(), cell.mass(), self.delta);
            }
            cell
        })
    }

    fn food_cells(&self) -> impl Iterator<Item = FoodCell> {
        self.prev.food_cells()
    }

    fn player_infos(&self) -> impl Iterator<Item = &PlayerInfo> {
        self.curr.player_infos()
    }

    fn view_area(&self) -> Circle {
        Self::lerp_circle(self.prev.view_area(), self.curr.view_area(), self.delta)
    }

    fn owner(&self) -> PlayerId {
        self.curr.owner()
    }
}

impl InterpolatedView<'_> {
    fn lerp_f64(prev: f64, curr: f64, delta: f64) -> f64 {
        prev + (curr - prev) * delta
    }

    fn lerp_point(prev: Point, curr: Point, delta: f64) -> Point {
        Point {
            x: Self::lerp_f64(prev.x, curr.x, delta),
            y: Self::lerp_f64(prev.y, curr.y, delta),
        }
    }

    fn lerp_circle(prev: Circle, curr: Circle, delta: f64) -> Circle {
        Circle {
            center: Self::lerp_point(prev.center, curr.center, delta),
            radius: Self::lerp_f64(prev.radius, curr.radius, delta),
        }
    }
}
