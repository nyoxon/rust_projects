#![allow(dead_code, unused_imports, unused_variables)]

use vectors::*;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use map::Name::*;
use map::Place;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Duration, Instant};
use rand::Rng;

static BONNIE_COLOR: Color = Color::RGB(0, 0, 255);
static BONNIE_INTERVAL: Duration = Duration::from_secs(5);
static BONNIE_AIS: [u32; 5] = [5, 7, 9, 12, 15];
static CHICA_COLOR: Color = Color::RGB(255, 255, 0);
static CHICA_INTERVAL: Duration = Duration::from_secs(5);
static CHICA_AIS: [u32; 5] = [3, 5, 11, 12, 13];
static FOXY_COLOR: Color = Color::RGB(100, 0, 0);
static FOXY_INTERVAL: Duration = Duration::from_secs(5);
static FOXY_AIS: [u32; 5] = [0, 5, 8, 11, 14];
static FREDDY_COLOR: Color = Color::RGB(88, 57, 39);
static FREDDY_INTERVAL: Duration = Duration::from_secs(3);
static FREDDY_AIS: [u32; 5] = [0, 0, 5, 8, 9];
static ANIMATRONIC_SIZE: u32 = 35;

type CompPlace = Rc<RefCell<Place>>;

#[derive(PartialEq, Clone)]
pub enum TypeAnimatronic {
    Bonnie,
    Chica,
    Foxy,
    Freddy,
}

use TypeAnimatronic::*;

pub struct Animatronic {
    ta: TypeAnimatronic,
    pub position: Vector,
    possible_places: Vec<CompPlace>,
    pub actual_place: CompPlace,
    moved: bool,
    ai: u32,
    night: u32,
    pub visible: bool,
    color: Color,
    interval: Duration,
    pub init_pos: Vector,
    time_to_kill: u64,
    pub init_to_kill: u64,
    timer: Rc<RefCell<Instant>>,
}

impl Animatronic {
    pub fn new
    (
        position: Vector,
        possible_places: Vec<CompPlace>,
        actual_place: CompPlace,
        night: u32,
        ta: TypeAnimatronic,
        timer: Rc<RefCell<Instant>>,
    ) -> Self 
    {
        let (color, ai, interval, time_to_kill) = match ta {
            Bonnie => (BONNIE_COLOR, BONNIE_AIS[(night - 1) as usize],
                       BONNIE_INTERVAL, 10),
            Chica => (CHICA_COLOR, CHICA_AIS[(night - 1) as usize],
                      CHICA_INTERVAL, 10),
            Foxy => (FOXY_COLOR, FOXY_AIS[(night - 1) as usize],
                     FOXY_INTERVAL, 30),
            Freddy => (FREDDY_COLOR, FREDDY_AIS[(night - 1) as usize],
                       FREDDY_INTERVAL, 7),
        };

        Self {
            ta,
            position: position.clone(),
            possible_places,
            actual_place,
            moved: false,
            ai,
            night,
            visible: false,
            color,
            interval,
            init_pos: position,
            time_to_kill,
            init_to_kill: 0,
            timer,
        }
    }


    pub fn change_night(&mut self, night: u32) {
        self.night = night;
    }

    fn moviment(&mut self) {
        let elapsed = self.timer.borrow().elapsed();

        if elapsed.as_secs() % self.interval.as_secs() == 0 &&
           elapsed.as_secs() > 0
        {   
            let mut rng = rand::thread_rng();
            if !self.moved {
                self.moved = true;
                let rng_number = rng.gen_range(1..=20);
                println!("{rng_number}, {ai}", ai = self.ai);
                if self.ai >= rng_number {
                    match &self.ta {
                        Foxy => (),
                        Freddy => {
                            let minutes = elapsed.as_secs() / 60;
                            if minutes >= 1 {
                                self.change_place();
                            } else {
                                println!(
                                "Freddy couldn't move: it's not 1 am yet or more");
                            }
                        },
                        _ => self.change_place(),
                    }
                }
            }
        } else {
            self.moved = false;
            return;
        }
    }

    fn check_kill(&self) -> bool {
        if self.init_to_kill == 0 {
            return false;
        }

        let elapsed = self.timer.borrow().elapsed();

        if elapsed.as_secs() - self.init_to_kill >= self.time_to_kill {
            return true;
        } else {
            return false;
        }
    }

    pub fn move_to_init(&mut self) {
        let stage_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Stage).unwrap());

        match &self.ta {
            Foxy => (),
            _ => {
                let stage_rc = Rc::clone(&self.possible_places.iter()
                               .find(|place| place.borrow().name == Stage)
                               .unwrap());
                self.actual_place = Rc::clone(&stage_rc);
                self.position = self.init_pos.clone();
            }
        }
    }

    fn bonnie_cp(&mut self) {
        let stage_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Stage).unwrap());
        let center_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Center).unwrap());
        let lh_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == LeftHall).unwrap());
        let lhc_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Lhc).unwrap());

        let center_position = center_rc.borrow().position.clone();
        let center_size = center_rc.borrow().size.clone();

        let lh_position = lh_rc.borrow().position.clone();
        let lh_size = lh_rc.borrow().size.clone();

        let lhc_position = lhc_rc.borrow().position.clone();
        let lhc_size = lhc_rc.borrow().size.clone();

        let actual_name = self.actual_place.borrow().name.clone();

        let mut rng = rand::thread_rng();
        let rand_number = rng.gen_range(1..=10);

        match actual_name {
            Stage => {
                if rand_number <= 8 {
                    self.actual_place = Rc::clone(&center_rc);
                    self.position = Vector::new
                    (
                        center_position.x + 
                        center_size.0 as f64 / 2.0 - (2 * ANIMATRONIC_SIZE) as f64, 
                        center_position.y + center_size.1 as f64 / 2.0,
                    )      
                } else {

                }
            },
            Center => {
                if rand_number <= 8 {
                    self.actual_place = Rc::clone(&lh_rc);
                    self.position = Vector::new
                    (
                        lh_position.x + 
                        lh_size.0 as f64 / 2.0 - ANIMATRONIC_SIZE as f64 / 2.0,
                        lh_position.y + lh_size.1 as f64 / 2.0,
                    )   
                } else {
                    self.actual_place = Rc::clone(&stage_rc);
                    self.position = self.init_pos.clone();
                }
            },
            LeftHall =>  {
                if rand_number <= 7 {
                    self.actual_place = Rc::clone(&lhc_rc);
                    self.position = Vector::new
                    (
                        lhc_position.x + 
                        lhc_size.0 as f64 / 2.0 + ANIMATRONIC_SIZE as f64,
                        lhc_position.y + lhc_size.1 as f64 / 2.0,
                    );

                    if self.init_to_kill == 0 {
                        self.init_to_kill = self.timer.borrow()
                                            .elapsed()
                                            .as_secs();
                    }
                } else {
                    self.actual_place = Rc::clone(&center_rc);
                    self.position = Vector::new
                    (
                        center_position.x + 
                        center_size.0 as f64 / 2.0 - (2 * ANIMATRONIC_SIZE) as f64, 
                        center_position.y + center_size.1 as f64 / 2.0,
                    )    
                }
            },
            Lhc => {
                self.check_kill();
            },
            _ => {},
        };

        println!("Bonnie changed to {:?}", self.actual_place.borrow().name);
    }

    fn chica_cp(&mut self) {
        let stage_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Stage).unwrap());
        let center_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Center).unwrap());
        let rh_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == RightHall).unwrap());
        let rhc_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Rhc).unwrap());

        let center_position = center_rc.borrow().position.clone();
        let center_size = center_rc.borrow().size.clone();

        let rh_position = rh_rc.borrow().position.clone();
        let rh_size = rh_rc.borrow().size.clone();

        let rhc_position = rhc_rc.borrow().position.clone();
        let rhc_size = rhc_rc.borrow().size.clone();

        let actual_name = self.actual_place.borrow().name.clone();

        let mut rng = rand::thread_rng();
        let rand_number = rng.gen_range(1..=10);
        match actual_name {
            Stage => {
                if rand_number <= 8 {
                    self.actual_place = Rc::clone(&center_rc);
                    self.position = Vector::new
                    (
                        center_position.x + 
                        center_size.0 as f64 - (3 * ANIMATRONIC_SIZE) as f64, 
                        center_position.y + center_size.1 as f64 / 2.0,
                    );
                } else {

                }
            },
            Center => {
                if rand_number <= 7 {
                    self.actual_place = Rc::clone(&rh_rc);
                    self.position = Vector::new
                    (
                        rh_position.x + 
                        rh_size.0 as f64 / 2.0 - ANIMATRONIC_SIZE as f64 / 2.0,
                        rh_position.y + rh_size.1 as f64 / 2.0,
                    );
                } else { 
                    self.actual_place = Rc::clone(&stage_rc);
                    self.position = self.init_pos.clone();
                }

            },
            RightHall =>  {
                if rand_number <= 7 {
                    self.actual_place = Rc::clone(&rhc_rc);
                    self.position = Vector::new
                    (
                        rhc_position.x + 
                        ANIMATRONIC_SIZE as f64,
                        rhc_position.y + rhc_size.1 as f64 / 2.0,
                    );

                    if self.init_to_kill == 0 {
                        self.init_to_kill = self.timer.borrow()
                                            .elapsed()
                                            .as_secs();
                    }
                } else {
                    self.actual_place = Rc::clone(&center_rc);
                    self.position = Vector::new
                    (
                        center_position.x + 
                        center_size.0 as f64 - (3 * ANIMATRONIC_SIZE) as f64, 
                        center_position.y + center_size.1 as f64 / 2.0,
                    );                    
                }
            },
            Rhc => {
                self.check_kill();
            },
            _ => {},
        };

        println!("Chica changed to {:?}", self.actual_place.borrow().name);
    }

    fn freddy_cp(&mut self) {
        if self.actual_place.borrow().clicked {
            println!("Freddy couldn't move: player was lightning it");
            return;
        }

        let stage_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Stage).unwrap());
        let center_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Center).unwrap());
        let rh_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == RightHall).unwrap());
        let rhc_rc = Rc::clone(&self.possible_places.iter()
                       .find(|place| place.borrow().name == Rhc).unwrap());

        let center_position = center_rc.borrow().position.clone();
        let center_size = center_rc.borrow().size.clone();

        let rh_position = rh_rc.borrow().position.clone();
        let rh_size = rh_rc.borrow().size.clone();

        let rhc_position = rhc_rc.borrow().position.clone();
        let rhc_size = rhc_rc.borrow().size.clone();

        let actual_name = self.actual_place.borrow().name.clone();

        let mut rng = rand::thread_rng();
        let rand_number = rng.gen_range(1..=10);
        match actual_name {
            Stage => {
                self.actual_place = Rc::clone(&center_rc);
                self.position = Vector::new
                (
                    self.init_pos.x, 
                    center_position.y + center_size.1 as f64 / 2.0,
                );
            },
            Center => {
                self.actual_place = Rc::clone(&rh_rc);
                self.position = Vector::new
                (
                    rh_position.x + 
                    rh_size.0 as f64 / 2.0 - ANIMATRONIC_SIZE as f64 / 2.0,
                    rh_position.y + rh_size.1 as f64 / 4.0,
                );
            },
            RightHall =>  {
                self.actual_place = Rc::clone(&rhc_rc);
                self.position = Vector::new
                (
                    rhc_position.x + 
                    ANIMATRONIC_SIZE as f64 * 2.0,
                    rhc_position.y + rhc_size.1 as f64 / 2.0,
                );

                if self.init_to_kill == 0 {
                    self.init_to_kill = self.timer.borrow()
                                        .elapsed()
                                        .as_secs();
                } 
            },
            Rhc => {
                self.check_kill();
            },
            _ => {},
        };

        println!("Freddy changed to {:?}", self.actual_place.borrow().name);
    }

    fn change_place(&mut self) {
        if self.possible_places.len() == 0 {
            return;
        }

        match &self.ta {
            Bonnie => self.bonnie_cp(),
            Chica => self.chica_cp(),
            Freddy => self.freddy_cp(),
            Foxy => (),
        }
    }

    fn change_visible(&mut self) {
        if self.actual_place.borrow().clicked {
            self.visible = true;
        } else {
            self.visible = false;
        }
    }

    fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        if !self.visible {
            return;
        }

        let rect = Rect::new
        (
            self.position.x as i32,
            self.position.y as i32,
            ANIMATRONIC_SIZE,
            ANIMATRONIC_SIZE,
        );

        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();
    }
}

pub struct Animatronics {
    pub animatronics: Vec<Animatronic>,
}

impl Animatronics {
    pub fn new() -> Self {
        Self {
            animatronics: vec![],
        }
    }

    pub fn add_animatronic(&mut self, animatronic: Animatronic) {
        self.animatronics.push(animatronic);
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        for animatronic in self.animatronics.iter() {
            animatronic.draw(canvas);
        }
    }

    pub fn moviment(&mut self) {
        for animatronic in self.animatronics.iter_mut() {
            animatronic.moviment();
        }
    }

    pub fn change_visible(&mut self) {
        for animatronic in self.animatronics.iter_mut() {
            animatronic.change_visible();
        }
    }

    pub fn check_kill(&self) -> bool {
        for animatronic in self.animatronics.iter() {
            if animatronic.check_kill() {
                return true;
            }
        }

        false
    }
}