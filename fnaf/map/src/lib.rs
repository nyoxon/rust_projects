#![allow(dead_code, unused_imports, unused_variables)]

use vectors::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Debug, Clone)]
pub enum Name {
    Office,
    Lhc,
    Rhc,
    RightHall,
    LeftHall,
    Stage,
    Center,
    Cove,
}

use Name::*;

#[derive(Debug, Clone)]
pub struct Place {
    pub position: Vector,
    color: Color,
    pub size: (u32, u32),
    pub name: Name,
    pub clicked: bool,
}

impl Place {
    pub fn new
    (
        position: Vector, 
        color: Color, 
        size: (u32, u32),
        name: Name,
    ) -> Self 
    {    
        Self {
            position,
            color,
            size,
            name,
            clicked: false,
        }
    }  

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let rect = Rect::new
        (
            self.position.x as i32,
            self.position.y as i32,
            self.size.0,
            self.size.1,
        );

        if self.name == Office {
            canvas.set_draw_color(self.color);
            canvas.fill_rect(rect).unwrap();  
            canvas.set_draw_color(Color::RGB(0, 0, 100));
            canvas.draw_rect(rect).unwrap();

            return;
        }

        if self.clicked {
            canvas.set_draw_color(self.color);
            canvas.fill_rect(rect).unwrap();
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0,));
            canvas.fill_rect(rect).unwrap();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 100));
        canvas.draw_rect(rect).unwrap();

    }
}

type CompPlace = Rc<RefCell<Place>>;

pub struct Map {
    places: Vec<CompPlace>,
}

impl Map {
    pub fn new() -> Self {  
        Self { places: vec![] }
    }

    pub fn add_place(&mut self, place: CompPlace) {
        self.places.push(place);
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        for place in self.places.iter() {
            place.borrow().draw(canvas);
        }
    }

    pub fn get(&self, key: &Name) -> Option<CompPlace> {
        Some(Rc::clone(self.places.iter()
        .find(|place| place.borrow().name == *key).unwrap()))
    }
}