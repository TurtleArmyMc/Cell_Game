use std::{cell::RefCell, rc::Rc};

use cell_game::{
    client_connection::{ClientConnection, PlayerInput},
    server::server_view::ServerView,
};

use crate::buffered_view::BufferedView;

pub struct LocalConnection {
    player_move_reader: Rc<RefCell<Option<PlayerInput>>>,
    view_buffer_writer: Rc<RefCell<Option<BufferedView>>>,
}

impl<'a> ClientConnection<'a> for LocalConnection {
    type V = ServerView<'a>;

    fn on_tick(&'a mut self, view: Self::V) -> Option<PlayerInput> {
        *self.view_buffer_writer.borrow_mut() = Some(BufferedView::new(&view));
        self.player_move_reader.borrow_mut().take()
    }
}

impl LocalConnection {
    pub fn new(
        player_move_reader: Rc<RefCell<Option<PlayerInput>>>,
        view_buffer_writer: Rc<RefCell<Option<BufferedView>>>,
    ) -> Self {
        Self {
            player_move_reader,
            view_buffer_writer,
        }
    }
}
