#![allow(dead_code, unused_imports, unused_variables)]

use features::*;
use vectors::*;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::Duration;
use std::ptr;

static CANVAS_SIZE: (u32, u32) = (800, 400);
static CANVAS_COLOR: Color = Color::RGB(0, 0, 0);
static BAR_COLOR: Color = Color::RGB(255, 255, 255);
static BAR_SIZE: (u32, u32) = (100, 20);
static BAR_POSITION: (u32, u32) = (CANVAS_SIZE.0  /  2 - BAR_SIZE.0  / 2,
                   CANVAS_SIZE.1 - 2 * BAR_SIZE.1);
static BALL_COLOR: Color = Color::RGB(255, 100, 50);
static BALL_SIZE: u32 = 15;
static BALL_POSITION: (u32, u32) = (CANVAS_SIZE.0 / 2 - BALL_SIZE / 2,
                    CANVAS_SIZE.1 / 2 - BALL_SIZE / 2);
static BARS_NUMBER_X: usize = (CANVAS_SIZE.0 / BAR_SIZE.0) as usize;
static BARS_NUMBER_Y: usize = 5;
static BARS_NUMBER: usize = BARS_NUMBER_X * BARS_NUMBER_Y;
static mut GAME_STATUS: GameStatus = Start;

#[derive(PartialEq)]
enum GameStatus {
    Start,
    Running,
    End,
}

use GameStatus::*;

fn create_context_and_canvas() -> (sdl2::Sdl, Canvas<sdl2::video::Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subystem = sdl_context.video().unwrap();
    let window = video_subystem
            .window("Pong game", CANVAS_SIZE.0, CANVAS_SIZE.1)
            .position_centered()
            .build()
            .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    (sdl_context, canvas)
}

fn main() {
    let (sdl, mut canvas) = create_context_and_canvas();
    let mut event_pump = sdl.event_pump().unwrap();
    let ttf = sdl2::ttf::init().unwrap();

    let mut breakable_bars = create_level();
    let bars_ptr = &mut breakable_bars as *mut Vec<Bar>;

    let mut bar = Bar::new
    (
        Vector::new(BAR_POSITION.0 as f64, BAR_POSITION.1 as f64),
        BAR_COLOR,
        BAR_SIZE,
        3.0,
        false,
        1,
    );

    let mut ball = Ball::new
    (
        Vector::new(BALL_POSITION.0 as f64, BALL_POSITION.1 as f64),
        BALL_COLOR,
        BALL_SIZE,
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    unsafe {
                        if GAME_STATUS!= Start {
                            continue;
                        } else if GAME_STATUS == End {
                            GAME_STATUS = Start;
                            continue;
                        }

                    }

                    unsafe {
                        GAME_STATUS = Running;
                    }

                    ball.start(Vector::new(-2.0, 2.0));
                }
                _ => {},
            }
            bar.move_by_keyboard(&event);
        }

        canvas.set_draw_color(CANVAS_COLOR);
        canvas.clear();

        for (index, breakbar) in breakable_bars.iter().enumerate() {
            if breakbar.health <= 0 {
                unsafe {
                    let element = (*bars_ptr).as_mut_ptr().add(index);
                    remove(bars_ptr, element);
                }
            } else {
                breakbar.draw(&mut canvas);
            }
        }

        bar.draw(&mut canvas);
        ball.draw(&mut canvas);

        unsafe {
            match GAME_STATUS {
                Start => start_game(&mut bar, &mut ball, &mut breakable_bars),
                Running => run_game(&mut bar, &mut ball, &mut breakable_bars),
                End => end_game(),
            }
        }

        canvas.present();
    }
}

unsafe fn remove<T>(collection_ptr: *mut Vec<T>, element_ptr: *mut T) {
    let collection = &mut *collection_ptr;

    let base_ptr = collection.as_mut_ptr();
    let index = element_ptr.offset_from(base_ptr) as usize;

    if index >= collection.len() {
        return;
    }

    collection.remove(index);
}

fn create_level() -> Vec<Bar> {
    let mut bars: Vec<Bar> = vec![];

    let x_num = BARS_NUMBER_X;
    let y_num = BARS_NUMBER_Y;

    let bar1_color = Color::RGB(0, 0, 255);
    let bar2_color = Color::RGB(0, 255, 0);
    let bar3_color = Color::RGB(255, 0, 0);

    for i in 0..y_num {
        for j in 0..x_num {
            match i {
                3 | 4 => {
                    let bar = Bar::new
                    (
                        Vector::new((BAR_SIZE.0 * j as u32) as f64,
                                    (BAR_SIZE.1 * i as u32) as f64),
                        bar1_color,
                        BAR_SIZE,
                        0.0,
                        true,
                        1,
                    );

                    bars.push(bar);
                },
                1 | 2 => {
                    let bar = Bar::new
                    (
                        Vector::new((BAR_SIZE.0 * j as u32) as f64,
                                    (BAR_SIZE.1 * i as u32) as f64),
                        bar2_color,
                        BAR_SIZE,
                        0.0,
                        true,
                        2,
                    );

                    bars.push(bar);  
                },
                0 => {
                    let bar = Bar::new
                    (
                        Vector::new((BAR_SIZE.0 * j as u32) as f64,
                                    (BAR_SIZE.1 * i as u32) as f64),
                        bar3_color,
                        BAR_SIZE,
                        0.0,
                        true,
                        3,
                    );

                    bars.push(bar);
                },
                _ => (),
            }
        }
    }

    bars
}

fn run_game
(
    bar: &mut Bar,
    ball: &mut Ball,
    breakbars: &mut Vec<Bar>,
)
{
    if ball.lost || breakbars.len() == 0 {
        unsafe {
            GAME_STATUS = End;
        }
        return;
    }

    for breakbar in breakbars {
        ball.check_collision_bar(breakbar);
    }

    ball.check_collision_bar(bar);
    bar.update_position();
    ball.update_position();
    ::std::thread::sleep(Duration::new(0, 1_000_000u32));
}

fn start_game
(
    bar: &mut Bar,
    ball: &mut Ball,
    breakbars: &mut Vec<Bar>,
)
{
    if breakbars.len() != BARS_NUMBER {
        *breakbars = create_level();
    }

    bar.velocity = Vector::new(0.0, 0.0);
    bar.position = Vector::new(BAR_POSITION.0 as f64, BAR_POSITION.1 as f64);

    ball.lost = false;
    ball.velocity = Vector::new(0.0, 0.0);
    ball.position = Vector::new(BALL_POSITION.0 as f64, BALL_POSITION.1 as f64);
}

fn end_game() {
    unsafe {
        GAME_STATUS = Start;
    }
}