use crate::{game_view::GameView, pos::Vec2};

#[derive(Clone)]
pub struct PlayerInput {
    /// A vector relative to the the view area of the player. The vector is
    /// centered on the center of the view area, and the magnitude is scaled
    /// by the view radius. This is done to keep the direction of player motion
    /// relatively consistent when working off of cached inputs.
    pub move_vec: Vec2,
}

pub trait ClientConnection<'a> {
    type V: GameView;

    fn on_tick(&mut self, view: Self::V) -> PlayerInput;
}
