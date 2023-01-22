use crate::pos::Point;

use super::cell::Cell;

pub struct FoodCell {
    pos: Point,
}

impl FoodCell {
    pub const MASS: f64 = 1.0;

    pub fn spawn_new(pos: Point) -> Self {
        Self { pos }
    }
}

impl Cell for FoodCell {
    fn pos(&self) -> crate::pos::Point {
        self.pos
    }

    fn mass(&self) -> f64 {
        Self::MASS
    }
}
