use std::f64;

use crate::pos::{Circle, Point};

pub trait Cell: Sized {
    const MASS_AREA_MULTIPLIER: f64 = f64::consts::PI;

    fn pos(&self) -> Point;

    fn mass(&self) -> f64;

    fn hitbox(&self) -> Circle {
        let radius = (self.mass() * Self::MASS_AREA_MULTIPLIER).sqrt();
        Circle {
            center: self.pos(),
            radius,
        }
    }
}
