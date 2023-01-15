use std::f64;

use cell_game::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    pos::Rect,
};
use wasm_bindgen::{JsCast, JsValue};

use crate::web_utils;

pub struct CanvasRender {
    cvs: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,

    visible_game_area: Option<Rect>,

    green_string: JsValue,
    red_string: JsValue,
}

impl CanvasRender {
    const CAMERA_AREA_SCALE: f64 = 0.9;

    pub fn new() -> Self {
        Self {
            cvs: web_utils::canvas(),
            ctx: web_utils::canvas_rendering_context_2d(),

            visible_game_area: None,

            green_string: JsValue::from_str(&RGBA::new(0, 255, 0, 255).to_string()),
            red_string: JsValue::from_str(&RGBA::new(255, 0, 0, 255).to_string()),
        }
    }

    pub fn visible_rect(&self) -> Option<Rect> {
        self.visible_game_area
    }

    pub fn render<'a, P, F, View>(&mut self, game: &View)
    where
        P: Iterator<Item = &'a PlayerCell>,
        F: Iterator<Item = &'a FoodCell>,
        View: GameView<'a, P, F>,
    {
        self.set_html_canvas_dimensions();
        self.clear_canvas();

        self.visible_game_area = game.view_area().map(|circle| {
            circle
                .fit_rect_within_circle(self.cvs.width() as f64 / self.cvs.height() as f64)
                .scale_centered(Self::CAMERA_AREA_SCALE)
        });

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

    fn render_cells<'a, P, F, View>(&self, game: &View)
    where
        P: Iterator<Item = &'a PlayerCell>,
        F: Iterator<Item = &'a FoodCell>,
        View: GameView<'a, P, F>,
    {
        if let Some(view_rect) = self.visible_game_area {
            let canvas_x_mid = self.cvs.width() as f64 / 2.0;
            let canvas_y_mid = self.cvs.height() as f64 / 2.0;
            let canvas_to_view_scale = self.cvs.width() as f64 / view_rect.width;

            self.ctx.set_stroke_style(&self.green_string);
            for p in game.player_cells() {
                let offset = p.pos().vec_to(view_rect.center()) * canvas_to_view_scale;
                let x = canvas_x_mid - offset.x;
                let y = canvas_y_mid - offset.y;
                self.draw_filled_circle(x, y, p.radius() * canvas_to_view_scale);
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
                let offset = f.pos().vec_to(view_rect.center()) * canvas_to_view_scale;
                let x = canvas_x_mid - offset.x;
                let y = canvas_y_mid - offset.y;
                self.draw_filled_circle(x, y, f.radius() * canvas_to_view_scale);
            }
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
