use crate::{
    ids::{IdGenerator, PlayerCellId, PlayerId},
    pos::{Point, Rect, Vec2},
};

use super::cell::Cell;

#[derive(Clone)]
pub struct PlayerCell {
    pos: Point,
    mass: f64,
    move_direction: Vec2,
    owner: PlayerId,
    id: PlayerCellId,
}

impl PlayerCell {
    pub const MAX_MOVE_SPEED: f64 = 10.0;
    pub const NEW_SPAWN_MASS: f64 = 20.0;
    pub const MASS_PERCENT_LOSS_PER_TICK: f64 = 0.01 / 60.0;

    pub(crate) fn new(
        pos: Point,
        owner: PlayerId,
        id_generator: &mut IdGenerator<PlayerCellId>,
    ) -> Self {
        Self {
            pos,
            mass: Self::NEW_SPAWN_MASS,
            move_direction: Vec2::ZERO,
            owner,
            id: id_generator.next(),
        }
    }

    pub fn move_towards_point(&mut self, pos: Point) {
        self.move_direction = self.pos.vec_to(pos);
        if self.move_direction.magnitude_squared() > Self::MAX_MOVE_SPEED * Self::MAX_MOVE_SPEED {
            self.move_direction = self.move_direction.normalize() * Self::MAX_MOVE_SPEED;
        }
    }

    pub fn move_player(&mut self, bounds: Rect) {
        self.pos = self.pos.offset(self.move_direction);
        self.pos.x = self.pos.x.max(bounds.min_x());
        self.pos.y = self.pos.y.max(bounds.min_y());
        self.pos.x = self.pos.x.min(bounds.max_x());
        self.pos.y = self.pos.y.min(bounds.max_y());
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
}

impl Cell for PlayerCell {
    fn pos(&self) -> Point {
        self.pos
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}
