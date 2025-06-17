use vectors::*;
use spring::*;
use rand::Rng;
use sdl2::pixels::Color;
use std::time::Duration;

static CANVAS_SIZE: (u32, u32) = (800, 600);
static CANVAS_COLOR: Color = Color::RGB(255, 255, 255);
static SPRING_COLOR: Color = Color::RGB(0, 0, 0);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Spring Simulation", CANVAS_SIZE.0, CANVAS_SIZE.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut springsystem = create_random_springs(10);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(CANVAS_COLOR);
        canvas.clear();

        springsystem.update();
        springsystem.draw(&mut canvas, SPRING_COLOR);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000u32));
    }
}


fn create_random_springs(num: usize) -> SpringSystem {
    let mut rng = rand::thread_rng();
    let mut old_positions = vec![];

    let mut springsystem = SpringSystem::new();

    for i in 0..num {
        if i == 0 {
            let origin = Vector::new
            (
                rng.gen_range(0..CANVAS_SIZE.0) as f64,
                rng.gen_range(0..CANVAS_SIZE.1) as f64,
            );

            let end = Vector::new
            (
                rng.gen_range(0..CANVAS_SIZE.0 as usize) as f64,
                rng.gen_range(0..CANVAS_SIZE.1 as usize) as f64,
            );

            old_positions.push(end.clone());

            let spring = Spring::new
            (
                origin,
                10.0,
                end,
                1.0,
                0.1,
                100.0,
            );

            springsystem.add_spring(spring);
            continue;
        }

        let end = Vector::new
        (
            rng.gen_range(0..CANVAS_SIZE.0 as usize) as f64,
            rng.gen_range(0..CANVAS_SIZE.1 as usize) as f64,
        );

        old_positions.push(end.clone());

        let spring = Spring::new
        (
            old_positions[i-1].clone(),
            10.0,
            end,
            1.0,
            0.1,
            100.0,
        );

        springsystem.add_spring(spring);
    }

    springsystem
}
