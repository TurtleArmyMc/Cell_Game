extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod renderer;
mod utils;
mod web_utils;

use cell_game::pos::Point;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_utils::JsResult;

#[wasm_bindgen(start)]
pub fn start() -> JsResult {
    let mut game = cell_game::server::GameServer::new();
    game.spawn_player();
    game.spawn_food();

    let renderer = renderer::CanvasRender::new();

    let move_to_reader = Rc::new(RefCell::new(None::<Point>));
    let move_to_writer = move_to_reader.clone();
    let mouse_move_callback_ref: Box<Closure<dyn FnMut(web_sys::MouseEvent)>> =
        Box::new(Closure::new(move |e: web_sys::MouseEvent| {
            let rect = web_utils::canvas()
                .dyn_into::<web_sys::Element>()
                .unwrap()
                .get_bounding_client_rect();

            let canvas = web_utils::canvas();
            let scale_x = (canvas.width() as f64) / rect.width();
            let scale_y = (canvas.height() as f64) / rect.height();
            *move_to_writer.borrow_mut() = Some(Point {
                x: (e.client_x() as f64 - rect.left()) * scale_x,
                y: (e.client_y() as f64 - rect.top()) * scale_y,
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
        if let Some(p) = move_to_reader.borrow_mut().take() {
            game.set_move_to(p);
        }
        game.tick();
        renderer.render(&game.game_view());

        web_utils::request_animation_frame(render_callback_ref_inner.borrow().as_ref().unwrap());
    });
    *render_callback_ref_outer.borrow_mut() = Some(render_callback);
    web_utils::request_animation_frame(render_callback_ref_outer.borrow().as_ref().unwrap());

    Ok(())
}
