#![allow(dead_code, unused_imports, unused_variables, static_mut_refs)]

use std::time::{Duration, Instant};
use vectors::*;
use map::*;
use player::*;
use animatronics::*;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::rect::Rect;
use std::rc::Rc;
use std::cell::RefCell;

use Name::*;
use TypeAnimatronic::*;

static CANVAS_SIZE: (u32, u32) = (800, 600);
static CANVAS_COLOR: Color = Color::RGB(0, 0, 0);
static OFFICE_COLOR: Color = Color::RGB(100, 100, 100);
static OFFICE_SIZE: (u32, u32) = (300, 150);
static OFFICE_POSITION: (f64, f64) = (CANVAS_SIZE.0 as f64 / 2.0 - 
                                      OFFICE_SIZE.0 as f64 / 2.0, 
                                      CANVAS_SIZE.1 as f64 / 2.0 - 
                                      OFFICE_SIZE.1 as f64 / 2.0 + 150.0);
static PLAYER_COLOR: Color = Color::RGB(255, 0, 0);
static PLAYER_SIZE: (u32, u32) = (50, 50);
static ANIMATRONIC_SIZE: u32 = 35;

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Start,
    Running,
    End,
}

use GameStatus::*;

struct TextHandler {
    font: String,
    size: u16,
    position: Vector,
    color: Color,
}

impl TextHandler {
    fn new
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

    fn draw_text
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

pub struct Manager {
    animatronics: Option<Animatronics>,
    map: Option<Map>,
    buttons: Option<Buttons>,
    timer: Rc<RefCell<Instant>>,
    canvas: Canvas<sdl2::video::Window>,
    ttf: Sdl2TtfContext,
    night_handler: TextHandler,
    hour_handler: TextHandler,
    night: u32,
}

impl Manager {
    pub fn new
    (
        canvas: Canvas<sdl2::video::Window>,
        ttf: Sdl2TtfContext,
    ) -> Self 
    {
        let night_handler = TextHandler::new
        (
            String::from("assets/font.TTF"),
            16,
            Vector::new(CANVAS_SIZE.0 as f64 - 70.0, 10.0),
            Color::RGB(255, 255, 255),
        );

        let hour_handler = TextHandler::new
        (
            String::from("assets/font.TTF"),
            16,
            Vector::new(CANVAS_SIZE.0 as f64 - 70.0, 40.0),
            Color::RGB(255, 255, 255),
        );

        Self {
            animatronics: None,
            map: None,
            buttons: None,
            timer: Rc::new(RefCell::new(Instant::now())),
            canvas,
            ttf,
            night_handler,
            hour_handler,
            night: 3,
        }
    }

    pub fn draw_canvas(&mut self) {
        self.canvas.set_draw_color(CANVAS_COLOR);
        self.canvas.clear();
    }

    pub fn present_canvas(&mut self) {
        self.canvas.present();
    }

    fn create_map(&mut self) {
        self.map = Some(Map::new());

        let office = Place::new
        (
            Vector::new(OFFICE_POSITION.0, OFFICE_POSITION.1),
            OFFICE_COLOR,
            OFFICE_SIZE,
            Office,
        );

        let left_hall_corner = Place::new
        (
            Vector::new(office.position.x - office.size.0 as f64 / 2.0,
                        office.position.y),
            OFFICE_COLOR,
            (office.size.0 / 2, 100),
            Lhc,
        );

        let right_hall_corner = Place::new
        (
            Vector::new(office.position.x + office.size.0 as f64,
                        office.position.y),
            OFFICE_COLOR,
            (office.size.0 / 2, 100),
            Rhc,
        );

        let hall_size = (left_hall_corner.size.0 - 50, 300);

        let left_hall = Place::new
        (
            Vector::new(left_hall_corner.position.x, 
                        left_hall_corner.position.y - 
                                    hall_size.1 as f64),
            OFFICE_COLOR,
            (hall_size.0, hall_size.1),
            LeftHall,
        );

        let right_hall = Place::new
        (
            Vector::new(right_hall_corner.position.x + 50.0 , 
                        right_hall_corner.position.y -
                                    hall_size.1 as f64),
            OFFICE_COLOR,
            (hall_size.0, hall_size.1),
            RightHall,
        );

        let center_size = (office.size.0, hall_size.1 - 50);

        let center = Place::new
        (
            Vector::new(office.position.x, office.position.y -
                                           hall_size.1 as f64),
            OFFICE_COLOR,
            center_size,
            Center,
        );

        let cove_size = (center_size.0 / 4, center_size.1 / 3);

        let cove = Place::new
        (
            Vector::new(center.position.x, center.position.y +
                                           (center_size.1 - cove_size.1) as f64),
            OFFICE_COLOR,
            cove_size,
            Cove,
        );

        let stage_size = (center_size.0 / 2, center_size.1 / 3);

        let stage = Place::new
        (
            Vector::new(center.position.x + stage_size.0 as f64 / 2.0,
                        center.position.y + 25.0),
            OFFICE_COLOR,
            stage_size,
            Stage,
        );

        let office_rc = Rc::new(RefCell::new(office));
        let center_rc = Rc::new(RefCell::new(center));
        let lhc_rc = Rc::new(RefCell::new(left_hall_corner));
        let rhc_rc = Rc::new(RefCell::new(right_hall_corner));
        let rh_rc = Rc::new(RefCell::new(right_hall));
        let lh_rc = Rc::new(RefCell::new(left_hall));
        let stage_rc = Rc::new(RefCell::new(stage));
        let cove_rc = Rc::new(RefCell::new(cove));

        self.map.as_mut().unwrap().add_place(Rc::clone(&office_rc));
        self.map.as_mut().unwrap().add_place(Rc::clone(&center_rc));
        self.map.as_mut().unwrap().add_place(Rc::clone(&lhc_rc));
        self.map.as_mut().unwrap().add_place(Rc::clone(&rhc_rc));
        self.map.as_mut().unwrap().add_place(Rc::clone(&rh_rc));
        self.map.as_mut().unwrap().add_place(Rc::clone(&lh_rc));
        self.map.as_mut().unwrap().add_place(Rc::clone(&stage_rc));
        self.map.as_mut().unwrap().add_place(Rc::clone(&cove_rc));
    }

    fn create_buttons(&mut self) {
        if self.map.is_none() {
            return;
        }

        self.buttons = Some(Buttons::new());
        let interval = (OFFICE_SIZE.0 / 7) as f64;
        let button_size = 25;

        let lhc_rc = self.map.as_mut().unwrap().get(&Lhc).unwrap();
        let center_rc = self.map.as_mut().unwrap().get(&Center).unwrap();
        let lh_rc = self.map.as_mut().unwrap().get(&LeftHall).unwrap();
        let rh_rc = self.map.as_mut().unwrap().get(&RightHall).unwrap();
        let rhc_rc = self.map.as_mut().unwrap().get(&Rhc).unwrap();
        let stage_rc = self.map.as_mut().unwrap().get(&Stage).unwrap();
        let cove_rc = self.map.as_mut().unwrap().get(&Cove).unwrap();

        let lhc_button = LightButton::new
        (
            Vector::new(OFFICE_POSITION.0 + interval / 3.5, OFFICE_POSITION.1),
            Rc::clone(&lhc_rc),
        );

        let center_button = LightButton::new
        (
            Vector::new(lhc_button.position.x + 2.0 * interval, 
                        lhc_button.position.y),
            Rc::clone(&center_rc),
        );

        let rhc_button = LightButton::new
        (
            Vector::new(lhc_button.position.x + 6.0 * interval, 
                        OFFICE_POSITION.1),
            Rc::clone(&rhc_rc),
        );

        let lh_button = LightButton::new
        (
            Vector::new(lhc_button.position.x + interval, 
                        lhc_button.position.y),
            Rc::clone(&lh_rc),
        );

        let rh_button = LightButton::new
        (
            Vector::new(lhc_button.position.x + 5.0 * interval, 
                        rhc_button.position.y),
            Rc::clone(&rh_rc),
        );

        let stage_button = LightButton::new
        (
            Vector::new(lhc_button.position.x + 4.0 * interval, 
                        lhc_button.position.y),
            Rc::clone(&stage_rc),
        );

        let cove_button = LightButton::new
        (
            Vector::new(lhc_button.position.x + 3.0 * interval, 
                        lhc_button.position.y),
            Rc::clone(&cove_rc),
        );

        let left_sound_button = SoundButton::new
        (
            Vector::new(lhc_button.position.x,
                        lhc_button.position.y + 80.0),
            Rc::clone(&lhc_rc),
        );

        let right_sound_button = SoundButton::new
        (
            Vector::new(rhc_button.position.x,
                        rhc_button.position.y + 80.0),
            Rc::clone(&rhc_rc),
        );

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(center_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(rhc_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(lh_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(lhc_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(rh_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(stage_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(cove_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(left_sound_button));

        self.buttons.as_mut().unwrap()
        .add_button(Box::new(right_sound_button));
    }

    fn create_animatronics(&mut self) {
        self.animatronics = Some(Animatronics::new());

        let stage_rc = self.map.as_mut().unwrap().get(&Stage).unwrap();
        let center_rc = self.map.as_mut().unwrap().get(&Center).unwrap();
        let lh_rc = self.map.as_mut().unwrap().get(&LeftHall).unwrap();
        let lhc_rc = self.map.as_mut().unwrap().get(&Lhc).unwrap();
        let rhc_rc = self.map.as_mut().unwrap().get(&Rhc).unwrap();
        let rh_rc = self.map.as_mut().unwrap().get(&RightHall).unwrap();
        let cove_rc = self.map.as_mut().unwrap().get(&Cove).unwrap();

        let bonnie = Animatronic::new
        (
            Vector::new(stage_rc.borrow().position.x + 10.0, 
                        stage_rc.borrow().position.y + 10.0),
            vec![Rc::clone(&stage_rc),
            Rc::clone(&center_rc),
            Rc::clone(&lh_rc),
            Rc::clone(&lhc_rc)],
            Rc::clone(&stage_rc),
            self.night,
            Bonnie,
            Rc::clone(&self.timer),
        );

        let chica = Animatronic::new
        (
            Vector::new(bonnie.position.x +
                stage_rc.borrow().size.0 as f64 - ANIMATRONIC_SIZE as f64 -
                20.0, 
                bonnie.position.y),
            vec![Rc::clone(&stage_rc),
            Rc::clone(&center_rc),
            Rc::clone(&rh_rc),
            Rc::clone(&rhc_rc)],
            Rc::clone(&stage_rc),
            self.night,
            Chica,
            Rc::clone(&self.timer),
        );

        let freddy = Animatronic::new
        (
            Vector::new(stage_rc.borrow().position.x + 
                        stage_rc.borrow().size.0 as f64 / 2.0
                        - ANIMATRONIC_SIZE as f64 / 2.0, 
                bonnie.position.y),
            vec![Rc::clone(&stage_rc),
            Rc::clone(&center_rc),
            Rc::clone(&rh_rc),
            Rc::clone(&rhc_rc)],
            Rc::clone(&stage_rc),
            self.night,
            Freddy,
            Rc::clone(&self.timer),
        );

        self.animatronics.as_mut()
        .unwrap().add_animatronic(bonnie);
        self.animatronics.as_mut()
        .unwrap().add_animatronic(chica);
        self.animatronics.as_mut()
        .unwrap().add_animatronic(freddy);
    }

    fn check_animatronics(&mut self) {
        for animatronic in self.animatronics.as_mut().unwrap()
                           .animatronics.iter_mut() {
            let name = animatronic.actual_place.borrow().name.clone();

            match name {
                Lhc | Rhc => {
                    animatronic.init_to_kill = 0;
                    animatronic.move_to_init();
                }
                _ => (),
            }
        }
    }

    pub fn buttons_on_click(&mut self, position: (i32, i32)) {
        if self.buttons.is_none() {
            return;
        }

        self.buttons.as_mut().unwrap().on_click(position);

        let ptr = self as *mut Self;

        for button in self.buttons.as_mut().unwrap().buttons.iter() {
            if button.get_type() == "SB" && button.get_clicked() {
                unsafe {
                    (*ptr).check_animatronics();
                }
            }
        }
    }

    pub fn create_game(&mut self) {
        self.init_timer();
        self.create_map();
        self.create_buttons();
        self.create_animatronics();
    }

    pub fn start_game(&mut self) 
    {
    }

    fn get_hour(&mut self) -> u64 {
        let elapsed = self.timer.borrow().elapsed();
        let minutes = elapsed.as_secs() / 60;

        match minutes {
            0 => 12,
            _ => minutes,
        }
    }

    pub fn run_game(&mut self) -> GameStatus
    {   
        if self.animatronics.as_ref().unwrap().check_kill() {
            return End;
        }

        if self.get_hour() == 6 {
            return End;
        }

        self.animatronics.as_mut().unwrap().moviment();
        self.animatronics.as_mut().unwrap().change_visible();
        self.map.as_ref().unwrap().draw(&mut self.canvas);
        self.animatronics.as_ref().unwrap().draw(&mut self.canvas);
        self.buttons.as_ref().unwrap().draw(&mut self.canvas);

        let night = format!("Night {}", self.night);
        let hour = format!("{} AM", self.get_hour());

        self.night_handler.draw_text(night.as_ref(), &mut self.canvas,
                                    &self.ttf);
        self.hour_handler.draw_text(hour.as_ref(), &mut self.canvas,
                                    &self.ttf);

        ::std::thread::sleep(Duration::from_millis(16));
        Running
    }

    pub fn end_game(&mut self) -> GameStatus {
        self.animatronics = None;
        self.buttons = None;
        self.map = None;

        Start
    }

    pub fn init_timer(&mut self) {
        self.timer = Rc::new(RefCell::new(Instant::now()));
    }
}
