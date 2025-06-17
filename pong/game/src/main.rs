#![allow(dead_code, unused_imports)]

use features::*;
use vectors::*;
use std::ptr;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::Duration;
use rand::Rng;

static CANVAS_SIZE: (u32, u32) = (800, 600);
static CANVAS_COLOR: Color = Color::RGB(0, 0, 0);
static BALL_POSITION: (f64, f64) = ((CANVAS_SIZE.0 / 2) as f64,
                                    (CANVAS_SIZE.1 / 2) as f64);
static BALL_COLOR: Color = Color::RGB(255, 0, 0);
static BAR_L_POSITION: (f64, f64) = (0 as f64,
                                    (CANVAS_SIZE.1 / 2 - 40) as f64);
static BAR_R_POSITION: (f64, f64) = ((CANVAS_SIZE.0 - 20) as f64,
                                    (CANVAS_SIZE.1 / 2 -40) as f64);
static BAR_COLOR: Color = Color::RGB(0, 0, 255);
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
    let (sdl_context, mut canvas) = create_context_and_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let ttf = sdl2::ttf::init().unwrap();


    let font_path = String::from("../assets/ASMAN.TTF");
    let mut texthandler1 = TextHandler::new
    (
        font_path.clone(),
        20,
        Vector::new(10.0, 10.0),
        Color::RGB(0, 255, 0),
    );

    let mut texthandler2 = TextHandler::new
    (
        font_path,
        20,
        Vector::new(CANVAS_SIZE.0 as f64 - 100.0, 10.0),
        Color::RGB(0, 255, 0),
    );

    let mut l_bar = Bar::new
    (
        Vector::new(BAR_L_POSITION.0, BAR_L_POSITION.1),
        texthandler1,
    );

    let mut r_bar = Bar::new
    (
        Vector::new(BAR_R_POSITION.0, BAR_R_POSITION.1),
        texthandler2,
    );

    let mut ball = Ball::new
    (
        Vector::new(BALL_POSITION.0, BALL_POSITION.1),
        vec![&mut l_bar as *mut Bar, &mut r_bar as *mut Bar],
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
                    }
                    let mut rng = rand::thread_rng();
                    let ball_velocity = Vector::new
                    (
                        rng.gen_range(1..3) as f64,
                        rng.gen_range(1..3) as f64,
                    );

                    ball.start(ball_velocity);
                    l_bar.start(Vector::new(0.0, -5.0));
                    r_bar.start(Vector::new(0.0, 5.0));
                    unsafe {
                        GAME_STATUS = Running;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    l_bar.velocity.y = -5.0;
                }
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    l_bar.velocity.y = 5.0;
                }
                _ => {},
            }
        }
        let content1 = format!("SCORE: {}", l_bar.get_score());
        let content2 = format!("SCORE: {}", r_bar.get_score());

        canvas.set_draw_color(CANVAS_COLOR);
        canvas.clear();
        ball.draw(&mut canvas, BALL_COLOR);
        l_bar.draw(&mut canvas, BAR_COLOR, &ttf);
        r_bar.draw(&mut canvas, BAR_COLOR, &ttf);

        unsafe {
            match GAME_STATUS {
                Start => start_game(&mut ball, &mut [&mut l_bar,
                                                     &mut r_bar]),
                Running => run_game(&mut ball, &mut [&mut l_bar,
                                                 &mut r_bar]),
                End => {
                    end_game();
                },
            }
        }

        canvas.present();
    }
}

fn start_game
(
    ball: &mut Ball,
    bars: &mut [&mut Bar], 
) 
{
    ball.velocity = Vector::new(0.0, 0.0);
    ball.position = Vector::new(BALL_POSITION.0, BALL_POSITION.1);

    if bars.is_empty() {
        return;
    }

    bars[0].velocity = Vector::new(0.0, 0.0);
    bars[0].change_score(0);
    bars[0].position = Vector::new(BAR_L_POSITION.0, BAR_L_POSITION.1);

    bars[1].velocity = Vector::new(0.0, 0.0);
    bars[1].change_score(0);
    bars[1].position = Vector::new(BAR_R_POSITION.0, BAR_R_POSITION.1);
}

fn run_game
(
    ball: &mut Ball, 
    bars: &mut [&mut Bar], 
) 
{
    ball.update_position(bars);
    bars[0].update_position(&ball, Who::Player);
    bars[1].update_position(&ball, Who::Ai);

    for bar in bars {
        if bar.get_score() == 2 {
            unsafe {
                GAME_STATUS = End;
            }
        }
    }

    ::std::thread::sleep(Duration::new(0, 1_000u32));
}

fn end_game() {
    unsafe {
        GAME_STATUS = Start;
    }
}