extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod renderer;
mod utils;
mod web_utils;

use cell_game::pos::{Point, Rect};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_utils::JsResult;

struct MoveState {
    move_to: Option<Point>,
    game_visible_area: Option<Rect>,
}
impl MoveState {
    fn new() -> Self {
        Self {
            move_to: None,
            game_visible_area: None,
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> JsResult {
    let mut game = cell_game::server::GameServer::new();
    game.spawn_player();
    for _ in 0..100 {
        game.spawn_food();
    }

    let mut renderer = renderer::CanvasRender::new();

    let move_state_render_ref = Rc::new(RefCell::new(MoveState::new()));
    let move_state_mouse_move_ref = move_state_render_ref.clone();
    let mouse_move_callback_ref: Box<Closure<dyn FnMut(web_sys::MouseEvent)>> =
        Box::new(Closure::new(move |e: web_sys::MouseEvent| {
            let mut move_state = move_state_mouse_move_ref.borrow_mut();
            if let Some(&visible_rect) = move_state.game_visible_area.as_ref() {
                let canvas_rect = web_utils::canvas()
                    .dyn_into::<web_sys::Element>()
                    .unwrap()
                    .get_bounding_client_rect();
                let canvas = web_utils::canvas();
                let canvas_scale_x = (canvas.width() as f64) / canvas_rect.width();
                let canvas_scale_y = (canvas.height() as f64) / canvas_rect.height();
                let canvas_x = (e.client_x() as f64 - canvas_rect.left()) * canvas_scale_x;
                let canvas_y = (e.client_y() as f64 - canvas_rect.top()) * canvas_scale_y;
                let x = visible_rect.min_x()
                    + ((canvas_x / canvas.width() as f64) * visible_rect.width);
                let y = visible_rect.min_y()
                    + ((canvas_y / canvas.height() as f64) * visible_rect.height);
                move_state.move_to = Some(Point { x, y });
            }
        }));
    web_utils::canvas()
        .add_event_listener_with_callback(
            "mousemove",
            Box::leak(mouse_move_callback_ref).as_ref().unchecked_ref(),
        )
        .expect("can not set listener mousemove on canvas");

    let render_callback_ref_outer = Rc::new(RefCell::new(None));
    let render_callback_ref_inner = render_callback_ref_outer.clone();
    let render_callback = Closure::new(move || {
        let mut move_state = move_state_render_ref.borrow_mut();
        if let Some(p) = move_state.move_to.take() {
            game.set_move_to(p);
        }
        game.tick();
        renderer.render(&game.game_view());

        move_state.game_visible_area = renderer.visible_rect();

        web_utils::request_animation_frame(render_callback_ref_inner.borrow().as_ref().unwrap());
    });
    *render_callback_ref_outer.borrow_mut() = Some(render_callback);
    web_utils::request_animation_frame(render_callback_ref_outer.borrow().as_ref().unwrap());

    Ok(())
}
