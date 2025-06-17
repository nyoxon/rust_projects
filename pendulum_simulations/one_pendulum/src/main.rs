extern crate sdl2;
extern crate vectors;
extern crate pendulum;

use vectors::*;
use pendulum::*;
use std::f64::consts::PI;
use sdl2::pixels::Color;
use std::time::Duration;

static CANVAS_SIZE: (u32, u32) = (800, 600);
static CANVAS_COLOR: Color =  Color::RGB(255, 255, 255);
static PENDULUM_COLOR: Color = Color::RGB(0, 0, 0);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
            .window("Pendulum Simulation", CANVAS_SIZE.0, CANVAS_SIZE.1)
            .position_centered()
            .build()
            .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut pendulum = Pendulum::new(
        Vector::new(400.0, 0.0),
        PI / 4.0,
        0.0,
        300.0,
        1.0,
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(CANVAS_COLOR);
        canvas.clear();

        pendulum.apply_force();
        pendulum.update_position();
        pendulum.draw(&mut canvas, PENDULUM_COLOR);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000u32));
    }
}
