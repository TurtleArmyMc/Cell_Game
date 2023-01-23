use rand::prelude::*;

use crate::{
    color::HSL,
    pos::{Point, Rect},
};

use super::cell::Cell;

#[derive(Clone)]
pub struct FoodCell {
    pos: Point,
    color: HSL,
}

impl FoodCell {
    pub const MASS: f64 = 1.0;

    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            color: HSL::new(random(), 245, 105),
        }
    }

    pub fn new_within(bounds: Rect) -> Self {
        Self::new(Point {
            x: bounds.min_x() + (bounds.width * random::<f64>()),
            y: bounds.min_y() + (bounds.height * random::<f64>()),
        })
    }

    pub fn color(&self) -> HSL {
        self.color
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
