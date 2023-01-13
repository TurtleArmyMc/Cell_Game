use crate::pos::{Point, Rect};

use super::cell::Cell;


pub struct PlayerCell {
    pos: Point,
    mass: f64,
    move_towards: Point,
}

impl PlayerCell {
    pub const MAX_MOVE_SPEED: f64 = 10.0;
    pub const NEW_SPAWN_MASS: f64 = 100.0;

    pub fn spawn_new(pos: Point) -> Self {
        Self {
            pos,
            mass: Self::NEW_SPAWN_MASS,
            move_towards: pos,
        }
    }

    pub fn set_move_towards(&mut self, pos: Point) {
        self.move_towards = pos;
    }

    pub fn move_player(&mut self, bounds: Rect) {
        self.pos = if self.pos.squared_dist_to(self.move_towards)
            < Self::MAX_MOVE_SPEED * Self::MAX_MOVE_SPEED
        {
            self.move_towards
        } else {
            self.pos
                .offset(self.pos.vec_to(self.move_towards).normalize() * Self::MAX_MOVE_SPEED)
        };
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
