use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x).powf(2.0) + (self.y).powf(2.0)).sqrt()
    }

    pub fn direction(&self, other: &Self) -> Self {
        let diff = other - self;
        let magnitude = diff.magnitude();

        Self::new(diff.x / magnitude, 
                 diff.y / magnitude)
    }

    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle(&self, other: &Self) -> f64 {
        (self.dot_product(other) / 
        (self.magnitude() * other.magnitude())).acos()
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x,
                    self.y + other.y)
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x,
                    self.y - other.y)
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector::new(self.x * other,
                    self.y * other)
    }
}