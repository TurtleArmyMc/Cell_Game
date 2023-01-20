use std::f64;

use cell_game::{
    cells::cell::Cell,
    game_view::GameView,
    pos::{Circle, Point},
};
use wasm_bindgen::{JsCast, JsValue};

use crate::{view_scaler::ViewScaler, web_utils};

pub struct CanvasRender {
    cvs: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,

    view_scaler: Option<ViewScaler>,

    green_string: JsValue,
    red_string: JsValue,
}

impl CanvasRender {
    pub fn new() -> Self {
        Self {
            cvs: web_utils::canvas(),
            ctx: web_utils::canvas_rendering_context_2d(),

            view_scaler: None,

            green_string: JsValue::from_str(&RGBA::new(0, 255, 0, 255).to_string()),
            red_string: JsValue::from_str(&RGBA::new(255, 0, 0, 255).to_string()),
        }
    }

    pub fn view_scaler(&self) -> Option<&ViewScaler> {
        self.view_scaler.as_ref()
    }

    pub fn render<'a, View>(&mut self, game: &'a View)
    where
        View: GameView<'a>,
    {
        self.set_html_canvas_dimensions();
        self.clear_canvas();

        self.view_scaler = ViewScaler::new(game, &self.cvs);

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

    fn render_cells<'a, View>(&self, game: &'a View)
    where
        View: GameView<'a>,
    {
        if let Some(scaler) = self.view_scaler() {
            self.ctx.set_stroke_style(&self.green_string);
            for p in game.player_cells() {
                self.draw_filled_circle(scaler.game_to_canvas_circle(p.hitbox()));
            }

            for f in game.food_cells() {
                self.ctx.set_stroke_style(
                    if game
                        .player_cells()
                        .any(|p| p.hitbox().overlaps_circle(f.hitbox()))
                    {
                        &self.red_string
                    } else {
                        &self.green_string
                    },
                );
                self.draw_filled_circle(scaler.game_to_canvas_circle(f.hitbox()));
            }
        }
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

    fn clear_canvas(&self) {
        self.ctx
            .clear_rect(0.0, 0.0, self.cvs.width() as f64, self.cvs.height() as f64);
    }
}

#[derive(Clone, Copy)]
struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl RGBA {
    fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl ToString for RGBA {
    fn to_string(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
