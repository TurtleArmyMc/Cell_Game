use std::{cell::RefCell, rc::Rc};

use cell_game::{
    client_connection::{ClientConnection, PlayerInput},
    server::server_view::ServerView,
};

use crate::view_history::ViewHistory;

pub struct LocalConnection {
    player_move_reader: Rc<RefCell<PlayerInput>>,
    view_history_writer: Rc<RefCell<ViewHistory>>,
}

impl<'a> ClientConnection<'a> for LocalConnection {
    type V = ServerView<'a>;

    fn on_tick(&'a mut self, view: Self::V) -> PlayerInput {
        self.view_history_writer.borrow_mut().update(&view);
        self.player_move_reader.borrow_mut().clone()
    }
}

impl LocalConnection {
    pub fn new(
        player_move_reader: Rc<RefCell<PlayerInput>>,
        view_history_writer: Rc<RefCell<ViewHistory>>,
    ) -> Self {
        Self {
            player_move_reader,
            view_history_writer,
        }
    }
}
