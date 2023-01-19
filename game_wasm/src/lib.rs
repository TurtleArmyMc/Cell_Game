extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod local_connection;
mod renderer;
mod utils;
mod web_utils;

use cell_game::pos::Point;
use local_connection::LocalConnection;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_utils::JsResult;

#[wasm_bindgen(start)]
pub fn start() -> JsResult {
    let mut game = cell_game::server::GameServer::new();
    for _ in 0..100 {
        game.spawn_food();
    }
    let renderer = renderer::CanvasRender::new();
    let canvas_move_writer = Rc::new(RefCell::new(None));

    let canvas_move_reader = canvas_move_writer.clone();
    let conn = LocalConnection::new(renderer, canvas_move_reader);
    game.add_connection(Box::new(conn));

    let mouse_move_callback_ref: Box<Closure<dyn FnMut(web_sys::MouseEvent)>> =
        Box::new(Closure::new(move |e: web_sys::MouseEvent| {
            let mut move_state = canvas_move_writer.borrow_mut();
            let canvas_rect = web_utils::canvas()
                .dyn_into::<web_sys::Element>()
                .unwrap()
                .get_bounding_client_rect();
            let canvas = web_utils::canvas();
            let canvas_scale_x = (canvas.width() as f64) / canvas_rect.width();
            let canvas_scale_y = (canvas.height() as f64) / canvas_rect.height();
            let canvas_x = (e.client_x() as f64 - canvas_rect.left()) * canvas_scale_x;
            let canvas_y = (e.client_y() as f64 - canvas_rect.top()) * canvas_scale_y;
            *move_state = Some(Point {
                x: canvas_x,
                y: canvas_y,
            });
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
        game.tick();
        web_utils::request_animation_frame(render_callback_ref_inner.borrow().as_ref().unwrap());
    });
    *render_callback_ref_outer.borrow_mut() = Some(render_callback);
    web_utils::request_animation_frame(render_callback_ref_outer.borrow().as_ref().unwrap());

    Ok(())
}
