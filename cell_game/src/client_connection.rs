use crate::{game_view::GameView, pos::Point};

pub struct PlayerInput {
    pub move_to: Point,
}

pub trait ClientConnection<'a> {
    type V: GameView<'a>;

    fn on_tick(&'a mut self, view: Self::V) -> Option<PlayerInput>;
}
