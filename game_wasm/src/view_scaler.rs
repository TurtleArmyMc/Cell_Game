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
    /// The amount of the view area that should be rendered on the canvas. This
    /// isn't the whole view area to avoid pop-in around the corners of the
    /// canvas if the game isn't local.
    const CAMERA_AREA_SCALE: f64 = 0.9;

    pub fn new<'a, V: GameView<'a>>(game_view: &V, cvs: &HtmlCanvasElement) -> Self {
        let visible_game_area = game_view
            .view_area()
            .fit_rect_within_circle(cvs.width() as f64 / cvs.height() as f64)
            .scale_centered(Self::CAMERA_AREA_SCALE);
        Self {
            visible_game_area,
            canvas_to_game_scale: cvs.width() as f64 / visible_game_area.width,
        }
    }

    pub fn canvas_to_game_x(&self, x: f64) -> f64 {
        self.visible_game_area.min_x() + (x / self.canvas_to_game_scale)
    }

    pub fn canvas_to_game_y(&self, y: f64) -> f64 {
        self.visible_game_area.min_y() + (y / self.canvas_to_game_scale)
    }

    pub fn game_to_canvas_x(&self, x: f64) -> f64 {
        (x - self.visible_game_area.min_x()) * self.canvas_to_game_scale
    }

    pub fn game_to_canvas_y(&self, y: f64) -> f64 {
        (y - self.visible_game_area.min_y()) * self.canvas_to_game_scale
    }

    pub fn canvas_to_game_pos(&self, Point { x: cvs_x, y: cvs_y }: Point) -> Point {
        Point {
            x: self.canvas_to_game_x(cvs_x),
            y: self.canvas_to_game_y(cvs_y),
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
            x: self.game_to_canvas_x(game_x),
            y: self.game_to_canvas_y(game_y),
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
