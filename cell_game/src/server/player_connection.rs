use crate::{client_connection::ClientConnection, ids::PlayerId};

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

    pub(crate) fn id(&self) -> PlayerId {
        self.id
    }

    pub(crate) fn connection(&mut self) -> &mut dyn ClientConnection<V = ServerView> {
        self.connection.as_mut()
    }
}
