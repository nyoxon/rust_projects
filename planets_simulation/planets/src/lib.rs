extern crate forces;
extern crate sdl2;

use forces::*;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use std::f32::consts::PI;

#[derive(Debug, PartialEq, Clone)]
pub struct Planet {
    pub position: Vector,
    pub velocity: Vector,
    radius: f64,
    mass: f64,
    color: Color,
    trajectory: Vec<Vector>,
}

impl Planet {
    const G: f64 = 100.0;

    pub fn new(
        position: Vector,
        velocity: Vector,
        radius: f64,
        mass: f64,
        color: Color,
    ) -> Self
    {
        Self {
            position,
            velocity,
            radius,
            mass,
            color,
            trajectory: vec![],
        }
    }

    pub fn update_position(&mut self) {
        self.position = &self.position + &self.velocity;

        self.trajectory.push(self.position.clone());
    }

    pub fn orbit_velocity(&mut self, other: &Self) {
        let distance = (&self.position - &other.position).magnitude();
        let velocity_magnitude = (Self::G * other.mass / distance).sqrt();
        let direction = self.position.direction(&other.position);
        let tangential_velocity = Vector::new(-direction.y, direction.x);

        self.velocity = &tangential_velocity * velocity_magnitude;
    }

    pub fn apply_force(&mut self, other: &Self) {
        let distance = (&other.position - &self.position).magnitude();

        if distance == 0.0 {
            return;
        }

        let magnitude = (Self::G * self.mass * other.mass) /
                        (distance.powf(2.0));

        let direction = &self.position.direction(&other.position);

        let force = direction * magnitude;
        let acceleration = &force * (1.0 / self.mass);
        self.velocity = &self.velocity + &acceleration;
    }

    pub fn draw_trajectory(&self, 
        renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {

        if self.trajectory.len() < 2 {
            return;
        }

        for i in 1..self.trajectory.len() {
            let start = &self.trajectory[i - 1];
            let end = &self.trajectory[i];

            renderer.set_draw_color(Color::RGBA(255, 255, 255, 255));
            renderer.draw_line((start.x as i32, start.y as i32),
                               (end.x as i32, end.y as i32)).unwrap();
        }

    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let num_segments = 360;
        let angle_step = 2.0 * PI / num_segments as f32;

        let pos_x = self.position.x as i32;
        let pos_y = self.position.y as i32;

        let radius = self.radius as f32;

        for i in 0..num_segments {
            let angle = angle_step * i as f32;
            let x = pos_x + (radius * angle.cos()) as i32;
            let y = pos_y + (radius * angle.sin()) as i32;

            canvas.set_draw_color(self.color);
            canvas.draw_line((pos_x, pos_y),
                             (x, y)).unwrap();
        }
    }
}

pub struct Planets {
    planets: Vec<Planet>,
}

impl Planets {
    pub fn new() -> Self {
        Self {
            planets: vec![],
        }
    }

    pub fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas){
        for planet in &self.planets {
            planet.draw(canvas);
        }
    }

    pub fn apply_force_sun(&mut self, sun: &Planet) {
        for planet in &mut self.planets {
            planet.apply_force(sun);
        }
    }

    pub fn apply_force_others(&mut self) {
        let mut copy = self.planets.clone();

        for planet in &mut self.planets {
            for p in &mut copy {
                if *planet != *p {
                    planet.apply_force(&p);
                }
            }
        }
    }

    pub fn update_position(&mut self) {
        for planet in &mut self.planets {
            planet.update_position();
        }
    }

    pub fn draw_trajectory(&self,
        renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for planet in &self.planets {
            planet.draw_trajectory(renderer);
        }
    }
}