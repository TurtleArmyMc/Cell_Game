use std::{
    iter::{repeat, Cloned, Map, Zip},
    slice::Iter,
};

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

    pub fn update<'a, V: GameView<'a>>(&mut self, view: &'a V) {
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

impl<'a> GameView<'a> for InterpolatedView<'a> {
    type P = Map<
        Zip<
            Zip<Cloned<std::slice::Iter<'a, PlayerCell>>, std::iter::Repeat<&'a ViewSnapshot>>,
            std::iter::Repeat<f64>,
        >,
        fn(((PlayerCell, &ViewSnapshot), f64)) -> PlayerCell,
    >;
    type F = Cloned<Iter<'a, FoodCell>>;
    type I = Iter<'a, PlayerInfo>;

    fn player_cells(&'a self) -> Self::P {
        fn try_lerp(((mut cell, prev), delta): ((PlayerCell, &ViewSnapshot), f64)) -> PlayerCell {
            if let Some(prev_cell) = prev
                .player_cells()
                .find(|prev_cell| prev_cell.id() == cell.id())
            {
                *cell.pos_mut() = InterpolatedView::lerp_point(prev_cell.pos(), cell.pos(), delta);
                *cell.mass_mut() = InterpolatedView::lerp_f64(prev_cell.mass(), cell.mass(), delta);
            }
            cell
        }

        self.curr
            .player_cells()
            .zip(repeat(self.prev))
            .zip(repeat(self.delta))
            .map(try_lerp as fn(((PlayerCell, &ViewSnapshot), f64)) -> PlayerCell)
    }

    fn food_cells(&'a self) -> Self::F {
        self.prev.food_cells()
    }

    fn player_infos(&'a self) -> Self::I {
        self.curr.player_infos()
    }

    fn view_area(&self) -> Option<Circle> {
        match (self.prev.view_area(), self.curr.view_area()) {
            (Some(prev), Some(curr)) => Some(Self::lerp_circle(prev, curr, self.delta)),
            (prev, curr) => curr.or(prev),
        }
    }

    fn owner(&self) -> PlayerId {
        self.curr.owner()
    }
}

impl<'a> InterpolatedView<'a> {
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
