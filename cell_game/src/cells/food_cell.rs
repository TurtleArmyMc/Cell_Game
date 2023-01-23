use rand::prelude::*;

use crate::pos::{Point, Rect};

use super::cell::Cell;

#[derive(Clone)]
pub struct FoodCell {
    pos: Point,
}

impl FoodCell {
    pub const MASS: f64 = 1.0;

    pub fn new(pos: Point) -> Self {
        Self { pos }
    }

    pub fn new_within(bounds: Rect) -> Self {
        Self::new(Point {
            x: bounds.min_x() + (bounds.width * random::<f64>()),
            y: bounds.min_y() + (bounds.height * random::<f64>()),
        })
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
