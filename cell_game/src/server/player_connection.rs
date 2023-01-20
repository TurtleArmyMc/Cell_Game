use crate::{client_connection::ClientConnection, player_info::PlayerId};

use super::server_view::ServerView;

pub(crate) struct PlayerConnection {
    connection: Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>,
    id: PlayerId,
}

impl PlayerConnection {
    pub(crate) fn new(
        connection: Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>,
        id: PlayerId,
    ) -> Self {
        Self { connection, id }
    }

    pub(crate) fn id(&self) -> u32 {
        self.id
    }

    pub(crate) fn connection(&mut self) -> &mut dyn ClientConnection<V = ServerView> {
        self.connection.as_mut()
    }
}
