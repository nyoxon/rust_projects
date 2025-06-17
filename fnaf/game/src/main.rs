#![allow(dead_code, unused_imports, unused_variables, static_mut_refs)]

use vectors::*;
use animatronics::*;
use map::*;
use map::Name::*;
use player::*;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use game::GameStatus;
use game::Manager;
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant};

use GameStatus::*;

static CANVAS_SIZE: (u32, u32) = (800, 600);
static mut GAME_STATUS: GameStatus = Start;

fn init_sdl2_and_create_canvas() -> (sdl2::Sdl, Canvas<sdl2::video::Window>) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("2d fnaf", CANVAS_SIZE.0, CANVAS_SIZE.1)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    (sdl, canvas)
}

fn main() {
    let (sdl, canvas) = init_sdl2_and_create_canvas();
    let mut event_pump = sdl.event_pump().unwrap();
    let ttf = sdl2::ttf::init().unwrap();

    let mut manager = Manager::new
    (
        canvas,
        ttf,
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    unsafe {
                        if GAME_STATUS != Start {
                            continue;
                        } else if GAME_STATUS == End {
                            GAME_STATUS = Start;
                            continue;
                        }

                        GAME_STATUS = Running;
                    }

                    manager.create_game();
                }
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    manager.buttons_on_click((x, y));
                }
                _ => {},
            }
        }

        manager.draw_canvas();

        unsafe {
            match GAME_STATUS {
                Start => manager.start_game(),
                Running => GAME_STATUS = manager.run_game(),
                End => GAME_STATUS = manager.end_game(),
            }
        }

        manager.present_canvas();
    }
}
