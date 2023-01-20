use std::{cell::RefCell, rc::Rc};

use cell_game::{
    client_connection::{ClientConnection, PlayerInput},
    pos::Point,
    server::server_view::ServerView,
};

use crate::renderer::CanvasRender;

pub struct LocalConnection {
    renderer: CanvasRender,
    canvas_move_reader: Rc<RefCell<Option<Point>>>,
}

impl<'a> ClientConnection<'a> for LocalConnection {
    type V = ServerView<'a>;

    fn on_tick(&'a mut self, view: Self::V) -> Option<PlayerInput> {
        self.renderer.render(&view);
        self.canvas_move_reader
            .borrow_mut()
            .take()
            .zip(self.renderer.view_scaler())
            .map(|(canvas_pos, scaler)| scaler.canvas_to_game_pos(canvas_pos))
            .map(|game_pos| PlayerInput { move_to: game_pos })
    }
}

impl LocalConnection {
    pub fn new(renderer: CanvasRender, canvas_move_reader: Rc<RefCell<Option<Point>>>) -> Self {
        Self {
            renderer,
            canvas_move_reader,
        }
    }
}
