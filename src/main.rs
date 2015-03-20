extern crate sdl2;

use graphics::sdl;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;
use sdl2::rect::Point;

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let backend = sdl::Backend();
    let renderer = graphics::newRenderer(backend, 800, 600);

    let mut drawer = backend.drawer;
    drawer.set_draw_color(Color::RGB(255, 0, 0));
    drawer.clear();
    drawer.set_draw_color(Color::RGB(0, 255, 0));
    for x in 200..600 {
        for y in 200..400 {
            drawer.draw_point(Point::new(x, y));
        }
    }
    drawer.present();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump();

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Q, .. } => {
                    running = false
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}
