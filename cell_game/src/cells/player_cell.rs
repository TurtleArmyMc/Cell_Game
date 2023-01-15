use crate::pos::{Point, Rect, Vec2};

use super::cell::Cell;

pub struct PlayerCell {
    pos: Point,
    mass: f64,
    move_direction: Vec2,
}

impl PlayerCell {
    pub const MAX_MOVE_SPEED: f64 = 10.0;
    pub const NEW_SPAWN_MASS: f64 = 100.0;

    pub fn spawn_new(pos: Point) -> Self {
        Self {
            pos,
            mass: Self::NEW_SPAWN_MASS,
            move_direction: Vec2::ZERO,
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
}

impl Cell for PlayerCell {
    fn pos(&self) -> Point {
        self.pos
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}
