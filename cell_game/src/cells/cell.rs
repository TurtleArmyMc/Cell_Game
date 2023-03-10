use std::f64;

use crate::pos::{Circle, Point};

pub trait Cell: Clone + Sized {
    const MASS_AREA_MULTIPLIER: f64 = f64::consts::PI * 5.0;

    fn pos(&self) -> Point;

    fn mass(&self) -> f64;

    fn radius_squared(&self) -> f64 {
        self.mass() * Self::MASS_AREA_MULTIPLIER
    }

    fn radius(&self) -> f64 {
        self.radius_squared().sqrt()
    }

    fn hitbox(&self) -> Circle {
        Circle {
            center: self.pos(),
            radius: self.radius(),
        }
    }
}
