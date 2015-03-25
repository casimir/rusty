extern crate clock_ticks;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate sdl2;

mod graphics;

use graphics::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() {
    env_logger::init().unwrap();
    let fps = 30;
    let delta: u64 = 1000000000 / fps;

    let mut gui = graphics::Backend::new(800, 600);
    let mut curr_time = clock_ticks::precise_time_ns();
    let mut last_time = curr_time;
    let mut running = true;

    colormap(&mut gui);
    let mut event_pump = gui.context.event_pump().unwrap();
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => running = false,
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    println!("Should export to JPEG");
                }
                _ => {}
            }
        }

        // The rest of the game loop goes here...

        if curr_time - last_time > delta {
            gui.paint();
            last_time = clock_ticks::precise_time_ns();
        }
        curr_time = clock_ticks::precise_time_ns();
    }
}

fn colormap(gui: &mut graphics::Backend) {
    let h = gui.canvas.h;
    let w = gui.canvas.w;
    for y in 0..gui.canvas.h {
        for x in 0..gui.canvas.w {
            gui.canvas.set(x,
                           y,
                           Color {
                               r: (x as f64 / w as f64 * 255.0) as u8,
                               g: (y as f64 / h as f64 * 255.0) as u8,
                               b: (((x * y) as f64 / (h.pow(2) + w.pow(2)) as f64).sqrt() *
                                   255.0) as u8,
                               a: 255,
                           })
        }
    }
    gui.paint();
}
