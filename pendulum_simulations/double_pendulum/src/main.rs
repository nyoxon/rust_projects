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
    // inicializa a biblioteca sdl2 e retorna o tipo
    // do sld_context (Sdl)

    let video_subsystem = sdl_context.video().unwrap();
    // inicializa a video subsystem de Sdl

    let window = video_subsystem
            .window("Pendulum Simulation", CANVAS_SIZE.0, CANVAS_SIZE.1)
            // inicializa um WindowsBuilder
            .position_centered()
            // centra a janela
            .build()
            // constroi a janela (retorna Window)
            .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    // inicializa um CanvasBuilder e já constroi um WindowCanvas

    let mut event_pump = sdl_context.event_pump().unwrap();
    // obtem o event pump do sdl
    // é permitido existir apenas um event_pump

    let mut double_pendulum = DoublePendulum::new(
        Vector::new(400.0, 300.0),
        (PI, PI / 2.0),
        (100.0, 100.0),
        (100.0, 0.5),
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            // iterador sobre os eventos
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                // clicar no X
                _ => {}
            }
        }

        canvas.set_draw_color(CANVAS_COLOR);
        // seta a cor atual de desenho do canvas

        canvas.clear();
        // desenha a tela inteira

        double_pendulum.apply_force();
        double_pendulum.update_position();
        double_pendulum.draw(&mut canvas, PENDULUM_COLOR);

        double_pendulum.draw_trajectory(&mut canvas);
        // println!("{}", double_pendulum.energy());

        canvas.present();
        // atualiza a tela com qualquer renderização
        // feita antes da chamada

        ::std::thread::sleep(Duration::new(0, 1_000_000u32));
    }
}
