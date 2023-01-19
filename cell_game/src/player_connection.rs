use crate::{client_connection::ClientConnection, player::PlayerId, server::ServerView};

pub struct PlayerConnection {
    connection: Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>,
    id: PlayerId,
}

impl PlayerConnection {
    pub fn new(
        connection: Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>,
        id: PlayerId,
    ) -> Self {
        Self { connection, id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn connection(&mut self) -> &mut dyn ClientConnection<V = ServerView> {
        self.connection.as_mut()
    }
}
