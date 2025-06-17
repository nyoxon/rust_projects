#![allow(dead_code, unused_imports, unused_variables, static_mut_refs)]

use vectors::*;
use map::*;
use map::Name::*;
use std::rc::Rc;
use std::cell::RefCell;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use sdl2::event::Event;

static LB_IDLE_COLOR: Color =  Color::RGB(0, 255, 0);
static BUTTON_SIZE: u32 = 25;
static LB_CLICKED_COLOR: Color = Color::RGB(0, 100, 0);
static SB_IDLE_COLOR: Color = Color::RGB(255, 0, 0);
static SB_CLICKED_COLOR: Color = Color::RGB(100, 0, 0);

pub trait Button {
    fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>);
    fn on_click(&mut self, mouse_position: (i32, i32));
    fn get_type(&self) -> String;
    fn get_clicked(&self) -> bool;
}

pub struct LightButton {
    pub position: Vector,
    color: Color,    
    clicked: bool,
    pub place: Rc<RefCell<Place>>,
}

pub struct SoundButton {
    position: Vector,
    color: Color,
    clicked: bool,
    pub place: Rc<RefCell<Place>>,
}

impl SoundButton {
    pub fn new
    (
        position: Vector,
        place: Rc<RefCell<Place>>,
    ) -> Self
    {
        Self {
            position,
            color: SB_IDLE_COLOR,
            clicked: false,
            place,
        }
    }
}

impl Button for SoundButton {
    fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let rect = Rect::new
        (
            self.position.x as i32,
            self.position.y as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rect(rect).unwrap();
    }

    fn on_click(&mut self, mouse_position: (i32, i32)) {
        let rect = Rect::new
        (
            self.position.x as i32,
            self.position.y as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        if rect.contains_point(mouse_position) {
            match self.clicked {
                true => {
                    self.color = SB_IDLE_COLOR;
                    self.clicked = false;
                },
                false => {
                    self.color = SB_CLICKED_COLOR;
                    self.clicked = true;
                },
            }
        }
    }

    fn get_type(&self) -> String {
        String::from("SB")
    }

    fn get_clicked(&self) -> bool {
        self.clicked
    }
}

impl LightButton {
    pub fn new
    (
        position: Vector,
        place: Rc<RefCell<Place>>,
    ) -> Self
    {
        Self {
            position,
            color: LB_IDLE_COLOR,
            clicked: false,
            place,
        }
    }

    fn change_place(&mut self) {
        let mut place = self.place.borrow_mut();
        match place.clicked {
            true => {
                place.clicked = false;
            },
            false => {
                place.clicked = true;
            },
        }
    }
}

impl Button for LightButton {
    fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let rect = Rect::new
        (
            self.position.x as i32,
            self.position.y as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rect(rect).unwrap();
    }

    fn on_click(&mut self, mouse_position: (i32, i32)) {
        let rect = Rect::new
        (
            self.position.x as i32,
            self.position.y as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        if rect.contains_point(mouse_position) {
            self.change_place();

            match self.clicked {
                true => {
                    self.color = LB_IDLE_COLOR;
                    self.clicked = false;
                },
                false => {
                    self.color = LB_CLICKED_COLOR;
                    self.clicked = true;
                },
            }
        }
    }

    fn get_type(&self) -> String {
        String::from("LB")
    }

    fn get_clicked(&self) -> bool {
        self.clicked
    }
}

pub struct Buttons {
    pub buttons: Vec<Box<dyn Button>>,
}

impl Buttons {
    pub fn new() -> Self {
        Self {
            buttons: vec![],
        }
    }

    pub fn add_button(&mut self, button: Box<dyn Button>) {
        self.buttons.push(button);
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        for button in self.buttons.iter() {
            button.draw(canvas);
        }
    }

    pub fn on_click(&mut self, mouse_position: (i32, i32)) {
        for button in self.buttons.iter_mut() {
            button.on_click(mouse_position);
        }
    }
}
