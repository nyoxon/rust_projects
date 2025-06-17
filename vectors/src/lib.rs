extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use std::ops::{Add, Sub, Mul};
use std::f64::consts::PI;

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

    pub fn inverse(&self) -> Self {
        self * (-1.0)
    }

    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        self.dot_product(self).sqrt()
    }

    pub fn direction(&self, other: &Self) -> Self {
        let diff = other - self;
        let magnitude = diff.magnitude();

        Self::new(diff.x / magnitude, 
                 diff.y / magnitude)
    }

    pub fn normalize(&self) -> Self {
        self * ( 1.0 / self.magnitude())
    }

    pub fn angle(&self, other: &Self) -> f64 {
        (self.dot_product(other) / 
        (self.magnitude() * other.magnitude())).acos()
    }

    pub fn angle_degrees(&self, other: &Self) -> f64 {
        self.angle(other) * 180.0 / PI
    }

    pub fn proj(&self, other: &Self) -> Self {
        other * ( (self.dot_product(other)) / (other.dot_product(other)))
    }

    pub fn draw_edges(start: &Vector, end: &Vector,
                      canvas: &mut Canvas<sdl2::video::Window>,
                      color: Color)
    {
        canvas.set_draw_color(color);
        canvas.draw_line((start.x as i32, start.y as i32),
                         (end.x as i32, end.y as i32)).unwrap();
    }

    pub fn draw_vector(&self,
                       canvas: &mut Canvas<sdl2::video::Window>,
                       color: Color)
    {
        canvas.set_draw_color(color);
        canvas.draw_line((0, 0), (self.x as i32, self.y as i32)).unwrap();
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

impl Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: &Vector) -> Vector {
        Vector::new(self * other.x,
                    self * other.y)
    }
}