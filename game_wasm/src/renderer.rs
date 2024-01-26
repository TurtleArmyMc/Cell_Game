use std::f64;

use cell_game::{
    cells::{cell::Cell, player_cell::PlayerCell},
    color::Color,
    game_view::GameView,
    player_info::PlayerInfo,
    pos::{Circle, Point, Vec2},
};
use wasm_bindgen::{JsCast, JsValue};

use crate::{view_scaler::ViewScaler, web_utils};

pub struct CanvasRender {
    cvs: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,

    view_scaler: Option<ViewScaler>,
}

impl CanvasRender {
    pub fn new() -> Self {
        Self {
            cvs: web_utils::canvas(),
            ctx: web_utils::canvas_rendering_context_2d(),

            view_scaler: None,
        }
    }

    pub fn view_scaler(&self) -> Option<&ViewScaler> {
        self.view_scaler.as_ref()
    }

    pub fn render(&mut self, game: &impl GameView) {
        self.set_html_canvas_dimensions();
        self.clear_canvas();

        self.view_scaler = Some(ViewScaler::new(game, &self.cvs));

        self.render_cells(game);
    }

    fn set_html_canvas_dimensions(&self) {
        let rect = self
            .cvs
            .clone()
            .dyn_into::<web_sys::Element>()
            .unwrap()
            .get_bounding_client_rect();
        self.cvs.set_width(rect.width() as u32);
        self.cvs.set_height(rect.height() as u32);
    }

    fn render_cells(&self, game: &impl GameView) {
        if let Some(scaler) = self.view_scaler() {
            for p in game.player_cells() {
                self.render_player_cell(
                    scaler,
                    &p,
                    game.player_infos().find(|i| i.id() == p.owner()).unwrap(),
                );
            }

            for f in game.food_cells() {
                self.set_stroke_color(f.color());
                self.draw_filled_circle(scaler.game_to_canvas_circle(f.hitbox()));
            }
        }
    }

    fn render_player_cell(&self, scaler: &ViewScaler, cell: &PlayerCell, info: &PlayerInfo) {
        self.set_stroke_color(info.color());
        self.draw_filled_circle(scaler.game_to_canvas_circle(cell.hitbox()));
        self.ctx.set_font("25px sans-serif");
        self.draw_centered_text(info.name(), scaler.game_to_canvas_pos(cell.pos()));
    }

    fn draw_filled_circle(
        &self,
        Circle {
            center: Point { x, y },
            radius,
        }: Circle,
    ) {
        self.ctx.set_line_cap("round");
        self.ctx.set_line_width(radius * 2.0);
        self.ctx.begin_path();
        self.ctx.move_to(x, y);
        self.ctx.line_to(x, y);
        self.ctx.stroke();
    }

    fn draw_centered_text(&self, text: &str, pos: Point) {
        let text_metrics = self.ctx.measure_text(text).expect("could not measure text");
        self.draw_text(
            text,
            pos.offset(Vec2 {
                x: -text_metrics.width() / 2.0,
                y: text_metrics.actual_bounding_box_descent(),
            }),
        );
    }

    /// Point is the top left corner of the text
    fn draw_text(&self, text: &str, Point { x, y }: Point) {
        self.ctx
            .fill_text(text, x, y)
            .expect("could not render text")
    }

    fn clear_canvas(&self) {
        self.ctx
            .clear_rect(0.0, 0.0, self.cvs.width() as f64, self.cvs.height() as f64);
    }

    fn set_stroke_color<T: Color>(&self, color: T) {
        self.ctx
            .set_stroke_style(&JsValue::from_str(&color.to_string()));
    }
}
