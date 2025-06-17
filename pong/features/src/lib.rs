#![allow(dead_code, unused_imports)]

use vectors::*;
use std::ptr;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::ttf::Sdl2TtfContext;

static CANVAS_SIZE: (u32, u32) = (800, 600);

pub struct TextHandler {
    font: String,
    size: u16,
    position: Vector,
    color: Color,
}

impl TextHandler {
    pub fn new
    (
        font: String,
        size: u16,
        position: Vector,
        color: Color,
    ) -> Self
    {
        Self {
            font,
            size,
            position,
            color,
        }
    }

    pub fn draw_text
    (
        &mut self, 
        text: &str,
        canvas: &mut Canvas<sdl2::video::Window>,
        ttf: &Sdl2TtfContext,
    ) 
    {
        let font = ttf.load_font(self.font.as_str(), self.size).unwrap();

        let surface = font
            .render(text)
            .blended(self.color)
            .unwrap();

        let texture_creator = canvas.texture_creator();
        let texture = surface
            .as_texture(&texture_creator)
            .unwrap();

        let target = Rect::new(self.position.x as i32, self.position.y as i32,
                               texture.query().width,
                               texture.query().height);
        canvas.copy(&texture, None, Some(target)).unwrap();
    }
}

pub enum Who {
    Player,
    Ai,
}

use Who::*;

pub struct Bar {
    pub position: Vector,
    pub velocity: Vector,
    score: u32,
    size: (u32, u32),
    texthandler: TextHandler,
}

impl Bar {
    pub fn new(position: Vector, texthandler: TextHandler) -> Self {
        Self {
            position,
            velocity: Vector::new(0.0, 0.0),
            score: 0,
            size: (20, 100),
            texthandler,
        }
    }

    fn invert_velocity(&mut self) {
        self.velocity.y *= -1.0;
    }

    pub fn start(&mut self, velocity: Vector) {
        self.velocity = velocity;
    }

    pub fn update_position(&mut self, ball: &Ball, who: Who) {
        self.check_collision_canvas();
        match who {
            Player => (),
            Ai => {
                let y_overlap = self.position.y - self.size.1 as f64;

                if y_overlap > 0.0 {
                    self.position.y = y_overlap;
                } else {
                    self.position.y = ball.position.y;
                }
            }
        }

        self.position = &self.position + &self.velocity;
    }

    fn check_collision_canvas(&mut self) {
        let height = self.size.1 as f64;

        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.invert_velocity()
        }

        if self.position.y + height >= CANVAS_SIZE.1 as f64 {
            self.position.y = CANVAS_SIZE.1 as f64 - height;
            self.invert_velocity()
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn change_score(&mut self, score: u32) {
        self.score = score;
    }

    pub fn draw(&mut self, canvas: &mut Canvas<sdl2::video::Window>,
                color: Color,
                ttf: &Sdl2TtfContext)
    {
        let rect = Rect::new(
            (self.position.x) as i32,
            (self.position.y) as i32,
            self.size.0,
            self.size.1,
        );

        canvas.set_draw_color(color);
        canvas.fill_rect(rect).unwrap();


        let text = format!("SCORE: {score}",
            score = self.score);
        self.texthandler.draw_text(text.as_str(), canvas, ttf);
    }
}

pub struct Ball {
    pub position: Vector,
    pub velocity: Vector,
    size: u32,
    pub bars: Vec<*mut Bar>,
}

impl Ball {
    pub fn new(position: Vector, bars: Vec<*mut Bar>) -> Self {
        Self {
            position,
            velocity: Vector::new(0.0, 0.0),
            size: 20,
            bars,
        }
    }

    fn invert_velocity(&mut self, dimension: u32) {
        match dimension {
            0 => self.velocity.x *= -1.0,
            1 => self.velocity.y *= -1.0,
            _ => (),
        }
    }

    pub fn start(&mut self, velocity: Vector) {
        self.velocity = velocity;
    }

    pub fn update_position(&mut self, bars: &mut [&mut Bar]) {
        self.check_collision_bar(bars[0]);
        self.check_collision_bar(bars[1]);
        self.check_collision_canvas();
        self.position = &self.position + &self.velocity;
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>,
                color: Color)
    {
        let rect = Rect::new(
            (self.position.x) as i32,
            (self.position.y) as i32,
            self.size,
            self.size,
        );

        canvas.set_draw_color(color);
        canvas.fill_rect(rect).unwrap();
    }

    fn check_collision_bar(&mut self, bar: &mut Bar) {
        let radius = self.size as f64;
        let width = bar.size.0 as f64;
        let height = bar.size.1 as f64;
        let bar_x = bar.position.x;
        let bar_y = bar.position.y;

        if !(self.position.x >= bar_x + width
            || self.position.x + radius <= bar_x
            || self.position.y >= bar_y + height
            || self.position.y + radius <= bar_y) 
        {   
            let x_overlap = self.position.x - bar_x;

            if x_overlap < 0.0 {
                self.invert_velocity(0);
                self.position.x = bar_x - width;
            } else {
                self.invert_velocity(0);
                self.position.x = bar_x + width;
            }
        } 
    }

    fn increment_score(&mut self, index: usize) {
        if self.bars.len() != 2 {
            return;
        }

        unsafe {
            if !self.bars.is_empty() 
            && !(self.bars[0]).is_null()
            && !(self.bars[1]).is_null() {
                (*(self.bars[index])).score += 1;
            }
        }
    }

    fn check_collision_canvas(&mut self) {
        let radius = self.size as f64 / 2.0;

        if self.position.x - radius <= 0.0 {
            self.position.x = 0.0 + radius;
            self.invert_velocity(0);

            self.increment_score(1);
        }

        if self.position.x + radius >= CANVAS_SIZE.0 as f64 {
            self.position.x = CANVAS_SIZE.0 as f64 - radius;
            self.invert_velocity(0);

            self.increment_score(0);
        }

        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.invert_velocity(1);
        }

        if self.position.y + radius >= CANVAS_SIZE.1 as f64 {
            self.position.y = CANVAS_SIZE.1 as f64 - radius;
            self.invert_velocity(1);
        }
    }
}