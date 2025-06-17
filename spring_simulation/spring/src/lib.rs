use vectors::*;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

static CANVAS_SIZE: (u32, u32) = (800, 600);

#[derive(Clone)]
pub struct Spring {
    pub origin: Vector,
    pub origin_mass: f64,
    pub origin_velocity: Vector,
    pub end: Vector,
    pub end_mass: f64,
    pub end_velocity: Vector,
    pub k: f64,
    pub rest_length: f64,
    pub current_length: f64,
}

impl Spring {
    const DELTA_TIME: f64 = 0.1;

    pub fn new
    (
        origin: Vector,
        origin_mass: f64,
        end: Vector,
        end_mass: f64,
        k: f64,
        rest_length: f64,
    ) -> Self
    {
        let current_length = (&origin - &end).magnitude();

        Self {
            origin,
            origin_mass,
            origin_velocity: Vector::new(0.0, 0.0),
            end,
            end_mass,
            end_velocity : Vector::new(0.0, 0.0),
            k,
            rest_length,
            current_length,
        }
    }

    pub fn check_bound(&mut self) {
        if self.origin.x < 0.0 || self.origin.x > CANVAS_SIZE.0 as f64 {
            self.origin_velocity.x *= -1.0;

            if self.origin.x < 0.0 {
                self.origin.x = 0.0;
            }

            if self.origin.x > CANVAS_SIZE.0 as f64 {
                self.origin.x = CANVAS_SIZE.0 as f64;
            }
        }

        if self.origin.y < 0.0 || self.origin.y > CANVAS_SIZE.1 as f64 {
            self.origin_velocity.y *= -1.0;

            if self.origin.y < 0.0 {
                self.origin.y = 0.0;
            }

            if self.origin.y > CANVAS_SIZE.1 as f64 {
                self.origin.y = CANVAS_SIZE.1 as f64;
            }
        }

        if self.end.x < 0.0 || self.end.x > CANVAS_SIZE.0 as f64 {
            self.end_velocity.x *= -1.0;

            if self.end.x < 0.0 {
                self.end.x = 0.0;
            }

            if self.end.x > CANVAS_SIZE.0 as f64 {
                self.end.x = CANVAS_SIZE.0 as f64;
            }
        }

        if self.end.y < 0.0 || self.end.y > CANVAS_SIZE.1 as f64 {
            self.end_velocity.y *= -1.0;

            if self.end.y < 0.0 {
                self.end.y = 0.0;
            }

            if self.end.y > CANVAS_SIZE.1 as f64 {
                self.end.y = CANVAS_SIZE.1 as f64;
            }
        }
    }

    pub fn update_positions(&mut self) {
        self.check_bound();

        self.origin = &self.origin + &(&self.origin_velocity * Self::DELTA_TIME);
        self.end = &self.end + &(&self.end_velocity * Self::DELTA_TIME);

        self.current_length = (&self.end - &self.origin).magnitude();
    }

    pub fn apply_forces(&mut self)
    {
        if self.current_length == self.rest_length {
            return;
        }

        let direction = (&self.end - &self.origin).normalize();
        let dx = self.current_length - self.rest_length;

        // |force|= k * |dx|
        // f = ma    =>    a = f / m

        let elastic_force = (-self.k * dx) * &direction;

        let force_origin = elastic_force.inverse();
        let force_end = elastic_force;

        let acceleration_origin = &force_origin * ( 1.0 / self.origin_mass );
        let acceleration_end = &force_end * ( 1.0 / self.end_mass);

        self.origin_velocity = &self.origin_velocity +
            &(&acceleration_origin * Self::DELTA_TIME);

        self.end_velocity = &self.end_velocity +
            &(&acceleration_end * Self::DELTA_TIME);
    }

    pub fn draw
    (
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        color: Color
    )
    {
        canvas.set_draw_color(color);
        canvas.draw_line((self.origin.x as i32, self.origin.y as i32),
                         (self.end.x as i32, self.end.y as i32)).unwrap();

        let origin_size = 10.0;
        let end_size = 10.0;

        let rect_origin = Rect::new
        (
            (self.origin.x - origin_size / 2.0) as i32,
            (self.origin.y - origin_size / 2.0) as i32,
            origin_size as u32,
            origin_size as u32,
        );

        let rect_end = Rect::new
        (
            (self.end.x - end_size / 2.0) as i32,
            (self.end.y - end_size / 2.0) as i32,
            end_size as u32,
            end_size as u32, 
        );

        canvas.fill_rect(rect_origin).unwrap();
        canvas.fill_rect(rect_end).unwrap();
    }
}


pub struct SpringSystem {
    pub springs: Vec<Spring>,
}

impl SpringSystem {
    pub fn new() -> Self {
        Self {
            springs: vec![],
        }
    }

    pub fn add_spring(&mut self, spring: Spring) {
        self.springs.push(spring);
    }

    pub fn update(&mut self) {
        let mut next_origins: Vec<Vector> = Vec
        ::with_capacity(self.springs.len());

        let mut next_velocities: Vec<Vector> = Vec
        ::with_capacity(self.springs.len());

        if self.springs.len() > 1 {
            let second_spring = &mut self.springs[1].clone();
            second_spring.apply_forces();
            second_spring.update_positions();

            self.springs[0].end = second_spring.origin.clone();
            self.springs[0].end_velocity = second_spring.origin_velocity.clone();
        }

        for (index, spring) in self.springs.iter_mut().enumerate() {
            if index > 0 {
                spring.origin = next_origins[index - 1].clone();
                spring.origin_velocity = next_velocities[index - 1].clone();
            }

            spring.apply_forces();

            next_origins.push(spring.end.clone());
            next_velocities.push(spring.end_velocity.clone());
        }

        for spring in &mut self.springs {
            spring.update_positions();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>, color: Color) {
        for spring in &self.springs {
            spring.draw(canvas, color);
        }
    }
}

pub struct SpringPendulum {
    pub origin: Vector,
    pub end: Vector,
    pub velocity: Vector,
    pub rest_length: f64,
    pub k: f64,
    pub mass: f64,
}
