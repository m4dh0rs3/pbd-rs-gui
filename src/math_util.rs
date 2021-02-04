//! Useful math tools: 2D Vector and it's operators, range projection
/// 2D Vector, f64
#[derive(Debug, Clone, Copy)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }

    /// From polar coordinates to vector
    pub fn from_polar(a: f64, m: f64) -> Vec2D {
        Vec2D {
            x: a.cos() * m,
            y: a.sin() * m,
        }
    }

    pub fn abs(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Normalize the vector -> v = v/|v|
    pub fn norm(&mut self) {
        let maq = self.abs();
        self.x /= maq;
        self.y /= maq;
    }

    pub fn dist(&self, other: &Vec2D) -> f64 {
        (
            (self.x - other.x).powi(2) +
            (self.y - other.y).powi(2)
        ).sqrt()
    }
}

use std::ops;

impl ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self, other: Vec2D) {
        *self = Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign<Vec2D> for Vec2D {
    fn sub_assign(&mut self, other: Vec2D) {
        *self = Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<f64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f64) -> Vec2D {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vec2D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f64> for Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: f64) -> Vec2D {
        Vec2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec2D {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// Range projection: f: {[a, b] → [c, d]; x ↦ x / (b-a) * (d-c) + c}
fn remap(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    x / (b-a) * (d-c) + c
}