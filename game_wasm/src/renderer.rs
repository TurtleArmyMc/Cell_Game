use std::f64;

use cell_game::{cells::cell::Cell, server::GameServer};
use wasm_bindgen::JsValue;

use crate::web_utils;

pub struct CanvasRender {
    cvs: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,

    green_string: JsValue,
    red_string: JsValue,
}

impl CanvasRender {
    pub fn new() -> Self {
        Self {
            cvs: web_utils::canvas(),
            ctx: web_utils::canvas_rendering_context_2d(),

            green_string: JsValue::from_str(&RGBA::new(0, 255, 0, 255).to_string()),
            red_string: JsValue::from_str(&RGBA::new(255, 0, 0, 255).to_string()),
        }
    }

    pub fn render(&self, game: &GameServer) {
        self.clear_canvas();
        self.ctx.set_stroke_style(&self.green_string);
        self.render_cells(game);
    }

    fn render_cells(&self, game: &GameServer) {
        self.ctx.set_stroke_style(&self.green_string);
        for p in game.player_cells() {
            self.draw_filled_circle(p.pos().x, p.pos().y, p.hitbox().radius);
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
            self.draw_filled_circle(f.pos().x, f.pos().y, f.hitbox().radius);
        }
    }

    fn draw_filled_circle(&self, x: f64, y: f64, radius: f64) {
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
