extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod buffered_view;
mod local_connection;
mod renderer;
mod utils;
mod view_scaler;
mod web_utils;

use cell_game::{client_connection::PlayerInput, pos::Point, server::game_server::GameServer};
use local_connection::LocalConnection;
use renderer::CanvasRender;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_utils::JsResult;

#[wasm_bindgen(start)]
pub fn start() -> JsResult {
    let mut game = GameServer::new();

    // The view buffer is a copy of the view of the last tick of the game.
    // This is kept buffered for rendering until the next tick.
    let view_buffer_reader = Rc::new(RefCell::new(None));
    let view_buffer_writer = view_buffer_reader.clone();
    // Keeps track of where the mouse was most recently moved to.
    // When rendering, this is mapped to a game position that is stored in
    // player_input_writer.
    let canvas_move_reader = Rc::new(RefCell::new(None));
    let canvas_move_writer = canvas_move_reader.clone();
    // Keeps track of the most recent input made while rendering, and read each
    // game tick.
    let player_input_reader = Rc::new(RefCell::new(None));
    let player_input_writer = player_input_reader.clone();

    let conn = LocalConnection::new(player_input_reader, view_buffer_writer);
    game.connect_player("Player".to_owned(), Box::new(conn));

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

    let mut renderer = CanvasRender::new();
    let render_callback_ref_outer = Rc::new(RefCell::new(None));
    let render_callback_ref_inner = render_callback_ref_outer.clone();
    let render_callback = Closure::new(move || {
        if let Some(view) = view_buffer_reader.borrow().as_ref() {
            renderer.render(view);
            if let Some(pos) = canvas_move_reader
                .borrow_mut()
                .take()
                .zip(renderer.view_scaler())
                .map(|(pos, scaler)| scaler.canvas_to_game_pos(pos))
            {
                *player_input_writer.borrow_mut() = Some(PlayerInput { move_to: pos });
            }
        }
        web_utils::request_animation_frame(render_callback_ref_inner.borrow().as_ref().unwrap());
    });
    *render_callback_ref_outer.borrow_mut() = Some(render_callback);
    web_utils::request_animation_frame(render_callback_ref_outer.borrow().as_ref().unwrap());
    web_utils::set_interval(
        Box::leak(Box::new(Closure::new(move || game.tick()))),
        1_000 / GameServer::TICK_RATE as i32,
    );

    Ok(())
}
