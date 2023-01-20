use cell_game::{
    game_view::GameView,
    pos::{Circle, Point, Rect},
};
use web_sys::HtmlCanvasElement;

/// Scales coordinates between
pub struct ViewScaler {
    visible_game_area: Rect,
    canvas_to_game_scale: f64,
}

impl ViewScaler {
    const CAMERA_AREA_SCALE: f64 = 0.9;

    pub fn new<'a, V: GameView<'a>>(game_view: &V, cvs: &HtmlCanvasElement) -> Option<Self> {
        game_view.view_area().map(|circle| {
            let visible_game_area = circle
                .fit_rect_within_circle(cvs.width() as f64 / cvs.height() as f64)
                .scale_centered(Self::CAMERA_AREA_SCALE);
            Self {
                visible_game_area,
                canvas_to_game_scale: cvs.width() as f64 / visible_game_area.width,
            }
        })
    }

    pub fn canvas_to_game_pos(&self, Point { x: cvs_x, y: cvs_y }: Point) -> Point {
        Point {
            x: self.visible_game_area.min_x() + (cvs_x / self.canvas_to_game_scale),
            y: self.visible_game_area.min_y() + (cvs_y / self.canvas_to_game_scale),
        }
    }

    pub fn game_to_canvas_pos(
        &self,
        Point {
            x: game_x,
            y: game_y,
        }: Point,
    ) -> Point {
        Point {
            x: (game_x - self.visible_game_area.min_x()) * self.canvas_to_game_scale,
            y: (game_y - self.visible_game_area.min_y()) * self.canvas_to_game_scale,
        }
    }

    pub fn canvas_to_game_circle(
        &self,
        Circle {
            center: cvs_center,
            radius: cvs_radius,
        }: Circle,
    ) -> Circle {
        Circle {
            center: self.canvas_to_game_pos(cvs_center),
            radius: cvs_radius / self.canvas_to_game_scale,
        }
    }

    pub fn game_to_canvas_circle(
        &self,
        Circle {
            center: game_center,
            radius: game_radius,
        }: Circle,
    ) -> Circle {
        Circle {
            center: self.game_to_canvas_pos(game_center),
            radius: game_radius * self.canvas_to_game_scale,
        }
    }
}
