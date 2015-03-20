extern crate sdl2;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;
use sdl2::rect::Point;

pub struct Backend {
    pub drawer: RenderDrawer
}

impl Backend {
    fn init(&self, x: i32, y: i32) {
        let window = match Window::new("rust-sdl2 demo: Video", WindowPos::PosCentered, WindowPos::PosCentered, x, y, OPENGL) {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err)
        };

        let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

        self.drawer = renderer.drawer();
    }

    fn render(c: &Canvas) {
        self.drawer.present();
    }
}
