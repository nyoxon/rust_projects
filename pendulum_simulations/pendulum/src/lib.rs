extern crate vectors;
extern crate sdl2;

use vectors::*;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Pendulum {
    pub origin: Vector,
    pub end: Vector,
    pub theta: f64,
    pub ang_velocity: f64,
    pub length: f64,
    pub mass: f64,
}

impl Pendulum {
    const g: f64 = 1.0e-1;

    pub fn new(origin: Vector,
               theta: f64,
               ang_velocity: f64,
               length: f64,
               mass: f64) -> Self 
    {
        let end_x = origin.x + length * theta.sin();
        let end_y = origin.y + length * theta.cos();

        Self {
            origin,
            end: Vector::new(end_x, end_y),
            theta,
            ang_velocity,
            length,
            mass,
        }
    }

    pub fn update_position(&mut self) {
        self.end.x = &self.origin.x + self.length * self.theta.sin();
        self.end.y = &self.origin.y + self.length * self.theta.cos();
    }

    pub fn apply_force(&mut self) {
        self.ang_velocity += -(Self::g * self.theta.sin()) / self.length;
        self.theta += self.ang_velocity;
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>,
                color: Color)
    {
        canvas.set_draw_color(color);
        canvas.draw_line((self.origin.x as i32, self.origin.y as i32),
                         (self.end.x as i32, self.end.y as i32)).unwrap();

        let rect = Rect::new(
            (self.end.x - 20.0 / 2.0) as i32,
            (self.end.y - 20.0 / 2.0) as i32,
            20.0 as u32,
            20.0 as u32,
        );

        canvas.set_draw_color(color);
        canvas.fill_rect(rect).unwrap();
    }
}

pub struct DoublePendulum {
    pub origins: (Vector, Vector),
    pub ends: (Vector, Vector),
    pub thetas: (f64, f64),
    pub ang_velocities: (f64, f64),
    pub lengths: (f64, f64),
    pub masses: (f64, f64),
    pub trajetories: Vec<Vec<Vector>>,
}

impl DoublePendulum {
    const g: f64 = 1.0;

    pub fn new(
        origin: Vector,
        thetas: (f64, f64),
        lengths: (f64, f64),
        masses: (f64, f64)
    ) -> Self
    {
        let end_x_0 = origin.x + lengths.0 * thetas.0.sin();
        let end_y_0 = origin.y + lengths.0 * thetas.0.cos();

        let end_0 = Vector::new(end_x_0, end_y_0);
        let origin_1 = end_0.clone();

        let end_x_1 = origin_1.x + lengths.1 * thetas.1.sin();
        let end_y_1 = origin_1.y + lengths.1 * thetas.1.cos();

        let ends = (Vector::new(end_x_0, end_y_0),
                    Vector::new(end_x_1, end_y_1));

        Self {
            origins: (origin, origin_1),
            ends,
            thetas,
            ang_velocities: (0.0, 0.0),
            lengths,
            masses,
            trajetories: vec![vec![], vec![]],
        }
    }

    pub fn update_position(&mut self) {
        self.ends.0.x = &self.origins.0.x + self.lengths.0 *
                        self.thetas.0.sin();
        self.ends.0.y = &self.origins.0.y + self.lengths.0 *
                        self.thetas.0.cos();

        self.ends.1.x = &self.origins.1.x + self.lengths.1 *
                        self.thetas.1.sin();
        self.ends.1.y = &self.origins.1.y + self.lengths.1 *
                        self.thetas.1.cos();

        self.origins.1 = self.ends.0.clone();
        self.trajetories[0].push(self.ends.0.clone());
        self.trajetories[1].push(self.ends.1.clone())
    }

    pub fn apply_force(&mut self) {
        let m1 = self.masses.0;
        let m2 = self.masses.1;
        let l1 = self.lengths.0;
        let l2 = self.lengths.1;
        let g = Self::g;
        let theta1 = self.thetas.0;
        let theta2 = self.thetas.1;
        let omega1 = self.ang_velocities.0;
        let omega2 = self.ang_velocities.1;

        let denom = 2.0 * m1 + m2 - m2 * (2.0 * (theta1 - theta2)).cos();

        if denom.abs() < 1e-10 {
            return;
        }

        let num1 = -g * (2.0 * m1 + m2) * theta1.sin()
            - m2 * g * (theta1 - 2.0 * theta2).sin()
            - 2.0 * (theta1 - theta2).sin()
                * m2
                * (omega2.powi(2) * l2 + omega1.powi(2) * l1 * (theta1 - theta2).cos());

        let acel_0 = num1 / (l1 * denom);

        let num2 = 2.0 * (theta1 - theta2).sin()
            * (omega1.powi(2) * l1 * (m1 + m2)
                + g * (m1 + m2) * theta1.cos()
                + omega2.powi(2) * l2 * m2 * (theta1 - theta2).cos());

        let acel_1 = num2 / (l2 * denom);

        let dt = 1.0;

        self.ang_velocities.0 += acel_0 * dt;
        self.ang_velocities.1 += acel_1 * dt;

        self.thetas.0 += self.ang_velocities.0 * dt;
        self.thetas.1 += self.ang_velocities.1 * dt;

        self.thetas.0 = self.thetas.0.rem_euclid(2.0 * std::f64::consts::PI);
        self.thetas.1 = self.thetas.1.rem_euclid(2.0 * std::f64::consts::PI);
    }

    pub fn energy(&self) -> f64 {
        let c1 = 0.5 * self.masses.0 * (self.ang_velocities.0 * self.lengths.0).powi(2);
        let c2 = 0.5 * self.masses.1 * (self.ang_velocities.1 * self.lengths.1).powi(2);

        let p1 = self.masses.0 * Self::g * self.ends.0.y;
        let p2 = self.masses.1 * Self::g * self.ends.1.y;

        c1 + c2 + p1 + p2
    }

    pub fn draw_trajectory(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        if self.trajetories[0].len() < 2 || self.trajetories[1].len() < 2{
            return;
        }

        for i in 1..self.trajetories[0].len() {
            let start = &self.trajetories[0][i - 1];
            let end = &self.trajetories[0][i];

            canvas.set_draw_color(Color::RGB(0, 0, 100));
            canvas.draw_line((start.x as i32, start.y as i32),
                             (end.x as i32, end.y as i32)).unwrap();
        }

        for i in 1..self.trajetories[1].len() {
            let start = &self.trajetories[1][i - 1];
            let end = &self.trajetories[1][i];

            canvas.set_draw_color(Color::RGB(0, 50, 255));
            canvas.draw_line((start.x as i32, start.y as i32),
                             (end.x as i32, end.y as i32)).unwrap();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>,
                color: Color)
    {
        canvas.set_draw_color(color);
        canvas.draw_line((self.origins.0.x as i32, self.origins.0.y as i32),
                         (self.ends.0.x as i32, self.ends.0.y as i32)).unwrap();

        canvas.draw_line((self.origins.1.x as i32, self.origins.1.y as i32),
                         (self.ends.1.x as i32, self.ends.1.y as i32)).unwrap();


        let rect_0 = Rect::new(
            (self.ends.0.x - 20.0 / 2.0) as i32,
            (self.ends.0.y - 20.0 / 2.0) as i32,
            20.0 as u32,
            20.0 as u32,
        );

        let rect_1 = Rect::new(
            (self.ends.1.x - 10.0 / 2.0) as i32,
            (self.ends.1.y - 10.0 / 2.0) as i32,
            10.0 as u32,
            10.0 as u32,
        );

        canvas.set_draw_color(color);
        canvas.fill_rect(rect_0).unwrap();
        canvas.fill_rect(rect_1).unwrap();
    }
}