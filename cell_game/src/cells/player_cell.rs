use crate::{
    ids::{IdGenerator, PlayerCellId, PlayerId},
    pos::{Point, Rect, Vec2},
    server::game_server::GameServer,
};

use super::cell::Cell;

#[derive(Clone)]
pub struct PlayerCell {
    pos: Point,
    mass: f64,
    owner: PlayerId,
    id: PlayerCellId,
}

impl PlayerCell {
    pub const MAX_MOVE_SPEED: f64 = 10.0;
    pub const NEW_SPAWN_MASS: f64 = 20.0;
    pub const MASS_PERCENT_LOSS_PER_TICK: f64 = 0.01 / GameServer::TICK_RATE as f64;

    pub(crate) fn new(
        pos: Point,
        owner: PlayerId,
        id_generator: &mut IdGenerator<PlayerCellId>,
    ) -> Self {
        Self {
            pos,
            mass: Self::NEW_SPAWN_MASS,
            owner,
            id: id_generator.next(),
        }
    }

    fn clamped_vec_to(&mut self, pos: Point) -> Vec2 {
        self.pos.vec_to(pos).max_magnitude(Self::MAX_MOVE_SPEED)
    }

    pub fn move_player(&mut self, move_to: Point, bounds: Rect) {
        let move_vec = self.clamped_vec_to(bounds.clamp_pos(move_to));
        self.pos = self.pos.offset(move_vec);
    }

    pub fn add_mass(&mut self, mass: f64) {
        self.mass += mass
    }

    pub fn lose_mass(&mut self) {
        self.mass = (self.mass * (1.0 - Self::MASS_PERCENT_LOSS_PER_TICK)).max(Self::NEW_SPAWN_MASS)
    }

    pub fn owner(&self) -> PlayerId {
        self.owner
    }

    pub fn id(&self) -> PlayerCellId {
        self.id
    }

    pub fn pos_mut(&mut self) -> &mut Point {
        &mut self.pos
    }

    pub fn mass_mut(&mut self) -> &mut f64 {
        &mut self.mass
    }
}

impl Cell for PlayerCell {
    fn pos(&self) -> Point {
        self.pos
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}
