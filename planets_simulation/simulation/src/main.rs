extern crate planets;
extern crate forces;
extern crate sdl2;

use planets::*;
use forces::*;

use sdl2::pixels::Color;
use std::time::Duration;

static CANVAS_COLOR: Color = Color::RGB(0, 0, 0);
static CANVAS_SIZE: (u32, u32) =  (800, 600);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
            .window("Orbit Simulation", CANVAS_SIZE.0, CANVAS_SIZE.1)
            .position_centered()
            .build()
            .unwrap();


    let sun = Planet::new(
              Vector::new(CANVAS_SIZE.0 as f64 / 2.0,
                          CANVAS_SIZE.1 as f64 / 2.0),
              Vector::new(0.0, 0.0),
              50.0,
              1.0,
              Color::RGB(255, 255, 0));


    let mut earth = Planet::new(
              Vector::new(300.0, CANVAS_SIZE.1 as f64 / 2.0 + -50.0),
              Vector::new(0.0, 0.0),
              10.0,
              1.0e-5,
              Color::RGB(0, 255, 0));
    earth.orbit_velocity(&sun);

    let mut mars = Planet::new(
              Vector::new(300.0, CANVAS_SIZE.1 as f64 / 2.0 + -80.0),
              Vector::new(0.0, 0.0),
              12.0,
              1.5e-5,
              Color::RGB(255, 0, 0));
    mars.orbit_velocity(&sun);

    let mut jupiter = Planet::new(
              Vector::new(100.0, CANVAS_SIZE.1 as f64 / 2.0 + 20.0),
              Vector::new(0.0, 0.0),
              30.0,
              2.0e-4,
              Color::RGB(255, 255, 100));
    jupiter.orbit_velocity(&sun);


    let mut neptune = Planet::new(
              Vector::new(700.0, CANVAS_SIZE.1 as f64 / 2.0 - 10.0),
              Vector::new(0.0, 0.0),
              18.0,
              3.0e-5,
              Color::RGB(255, 0, 0));
    neptune.orbit_velocity(&sun);

    let mut planets = Planets::new();
    planets.add_planet(earth);
    planets.add_planet(mars);
    planets.add_planet(jupiter);
    planets.add_planet(neptune);

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        planets.apply_force_sun(&sun);
        planets.apply_force_others();
        planets.update_position();

        canvas.set_draw_color(CANVAS_COLOR);
        canvas.clear();

        planets.draw_trajectory(&mut canvas);
        planets.draw(&mut canvas);
        sun.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }

}
