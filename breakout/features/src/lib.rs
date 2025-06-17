#![allow(dead_code, unused_imports, unused_variables)]

use vectors::*;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cmp::max;

static CANVAS_SIZE: (u32, u32) = (800, 400);

#[derive(Clone)]
pub struct Bar {
    pub position: Vector,
    pub velocity: Vector,
    color: Color,
    size: (u32, u32),
    speed: f64,
    pub breakable: bool,
    pub health: i32,
}

impl Bar {
    pub fn new
    (
        position: Vector,
        color: Color,
        size: (u32, u32),
        speed: f64,
        breakable: bool,
        health: i32,
    ) -> Self
    {
        Self {
            position,
            velocity: Vector::new(0.0, 0.0),
            color,
            size,
            speed,
            breakable,
            health,
        }
    }

    pub fn update_position(&mut self) {
        self.check_collision_bounds();
        self.position = &self.position + &self.velocity;
    }

    pub fn move_by_keyboard(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                self.velocity.x = -self.speed;
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                self.velocity.x = self.speed;
            }
            _ => {},
        }
    }

    fn check_collision_bounds(&mut self) {
        let x = self.position.x;
        let y = self.position.y;
        let size_x = self.size.0 as f64;
        let size_y = self.size.1 as f64;
        let width = CANVAS_SIZE.0 as f64;
        let height = CANVAS_SIZE.1 as f64;

        if x <= 0.0 {
            self.position.x = 0.0;
        }

        if x >= width - size_x {
            self.position.x = width - size_x;
        }

        if y <= 0.0 {
            self.position.y = 0.0;
        }

        if y >= height - size_y {
            self.position.y = height - size_y;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let rect = Rect::new
        (
            (self.position.x) as i32,
            (self.position.y) as i32,
            self.size.0,
            self.size.1,
        );

        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();

        if self.breakable {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.draw_rect(rect).unwrap();
        }
    }
}

pub struct Ball {
    pub position: Vector,
    pub velocity: Vector,
    color: Color,
    size: u32,
    pub lost: bool,
}

impl Ball {
    pub fn new
    (
        position: Vector,
        color: Color,
        size: u32,
    ) -> Self
    {
        Self {
            position,
            velocity: Vector::new(0.0, 0.0),
            color,
            size,
            lost: false,
        }
    }

    pub fn start(&mut self, velocity: Vector) {
        self.velocity = velocity;
    }

    pub fn update_position(&mut self) {
        self.check_collision_bounds();
        self.position = &self.position + &self.velocity;
    }

    fn check_collision_bounds(&mut self) {
        let x = self.position.x;
        let y = self.position.y;
        let size = self.size as f64;
        let width = CANVAS_SIZE.0 as f64;
        let height = CANVAS_SIZE.1 as f64;

        if x <= 0.0 {
            self.position.x = 0.0;
            self.velocity.x *= -1.0;
        }

        if x >= width - size {
            self.position.x = width - size;
            self.velocity.x *= -1.0;
        }

        if y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y *= -1.0;
        }

        if y >= height - size {
            self.position.y = height - size;
            self.velocity.y *= -1.0;
            self.lost = true;
        }
    }

    pub fn check_collision_bar(&mut self, bar: &mut Bar) {
        let size = self.size as f64;
        let width = bar.size.0 as f64;
        let height = bar.size.1 as f64;
        let bar_x = bar.position.x;
        let bar_y = bar.position.y;

        if self.position.x + size >= bar_x &&
            self.position.x <= bar_x + width &&
            self.position.y + size >= bar_y &&
            self.position.y <= bar_y + height
        {   
            if bar.breakable {
                bar.health -= 1;

                if bar.health > 0 {
                    let old_r = bar.color.r;
                    let old_g = bar.color.g;
                    let old_b = bar.color.b;

                    let new_r = if old_r > 0 {
                        max(old_r - 50, 0)
                    } else {
                        old_r
                    };

                    let new_g = if old_g > 0 {
                        max(old_g - 50, 0)
                    } else {
                        old_g
                    };

                    let new_b = if old_b > 0 {
                        max(old_b - 50, 0)
                    } else {
                        old_b
                    };

                    bar.color = Color::RGB(new_r, new_g, new_b);
                }
            }

            let center_self = (self.position.x + size / 2.0,
                               self.position.y + size / 2.0);
            let center_bar = (bar_x + width / 2.0,
                              bar_y + height / 2.0);

            let delta_y = center_self.1 - center_bar.1;
            let y_overlap = size / 2.0 + height / 2.0;

            if self.velocity.y > 0.0 && delta_y < 0.0 {
                if delta_y.abs() <= y_overlap {
                    self.position.y = bar_y - height;
                    self.velocity.y *= -1.0;

                    if !bar.breakable {
                        if self.position.x <= bar_x + width / 2.0 {
                            if self.velocity.x > 0.0 {
                                self.velocity.x *= -1.0;
                            }
                        } else {
                            if self.velocity.x < 0.0 {
                                self.velocity.x *= -1.0;
                            }
                        }
                    }
                }
            } else if self.velocity.y < 0.0 && delta_y > 0.0 {
                if delta_y.abs() <= y_overlap {
                    self.position.y = bar_y + height;
                    self.velocity.y *= -1.0;

                    if !bar.breakable {
                        if self.position.x <= bar_x + width / 2.0 {
                            if self.velocity.x > 0.0 {
                                self.velocity.x *= -1.0;
                            }
                        } else {
                            if self.velocity.x < 0.0 {
                                self.velocity.x *= -1.0;
                            }
                        }    
                    }
                }
            }
        } 
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>)
    {
        let rect = Rect::new(
            (self.position.x) as i32,
            (self.position.y) as i32,
            self.size,
            self.size,
        );

        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();
    }
}
