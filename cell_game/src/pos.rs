use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn squared_dist_to(self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn dist_to(self, other: Self) -> f64 {
        self.squared_dist_to(other).sqrt()
    }

    pub fn offset(self, offset: Vec2) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    pub fn vec_to(self, other: Self) -> Vec2 {
        Vec2 {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn magnitude_squared(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, scale: f64) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub top_left: Point,
    /// Must be positive
    pub width: f64,
    /// Must be positive
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            top_left: Point { x, y },
            width,
            height,
        }
    }

    pub fn contains_point(self, p: Point) -> bool {
        self.top_left.x <= p.x
            && p.x <= self.top_left.x + self.width
            && self.top_left.y <= p.y
            && p.y <= self.top_left.y + self.height
    }

    pub fn center(self) -> Point {
        self.top_left.offset(Vec2 {
            x: self.width / 2.0,
            y: self.height / 2.0,
        })
    }

    pub fn bottom_right(self) -> Point {
        self.top_left.offset(Vec2 {
            x: self.width,
            y: self.height,
        })
    }

    pub fn min_x(self) -> f64 {
        self.top_left.x
    }

    pub fn min_y(self) -> f64 {
        self.top_left.y
    }

    pub fn max_x(self) -> f64 {
        self.top_left.x + self.width
    }

    pub fn max_y(self) -> f64 {
        self.top_left.y + self.height
    }
}

#[derive(Clone, Copy)]
pub struct Circle {
    pub center: Point,
    /// Radius must be positive
    pub radius: f64,
}

impl Circle {
    pub fn overlaps_circle(self, other: Self) -> bool {
        let radius_sum = self.radius + other.radius;
        self.center.squared_dist_to(other.center) <= radius_sum * radius_sum
    }

    pub fn scale_centered(self, factor: f64) -> Self {
        Self {
            center: self.center,
            radius: self.radius * factor,
        }
    }
}
