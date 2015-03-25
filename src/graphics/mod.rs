extern crate sdl2;

use sdl2::pixels::Color::RGB;
use sdl2::render::Renderer;
use sdl2::rect::Point;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy)]
pub enum Pixel {
    Data(Color),
    Blank,
}

pub struct Canvas {
    pub w: usize,
    pub h: usize,
    pixels: Vec<Pixel>,
}

impl Canvas {
    fn get(&self, x: usize, y: usize) -> Pixel {
        if x > self.w || y > self.h {
            error!("Invalid coordinates: ({}, {})", x, y);
            Pixel::Blank
        } else {
            self.pixels[x + y * self.w]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        if x > self.w || y > self.h {
            error!("Invalid coordinates: ({}, {})", x, y);
        } else {
            self.pixels[x + y * self.w] = Pixel::Data(c)
        }
    }
}

pub struct Backend {
    pub canvas: Canvas,
    pub context: sdl2::Sdl,
    renderer: Renderer<'static>,
}

impl Backend {
    pub fn new(x: u32, y: u32) -> Backend {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let window = video.window("Rusty", x, y)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let renderer = window.renderer().build().unwrap();

        let xs = x as usize;
        let ys = y as usize;
        Backend {
            context: sdl_context,
            renderer: renderer,
            canvas: Canvas {
                w: xs,
                h: ys,
                pixels: vec![Pixel::Blank; xs * ys],
            },
        }
    }

    pub fn paint(&mut self) {
        let ref mut rend = self.renderer;
        for x in 0..self.canvas.w {
            for y in 0..self.canvas.h {
                match self.canvas.get(x, y) {
                    Pixel::Data(color) => {
                        rend.set_draw_color(RGB(color.r, color.g, color.b));
                        rend.draw_point(Point::new(x as i32, y as i32));
                    }
                    Pixel::Blank => {}
                };
            }
        }
        rend.present();
        // XXX show metrics
    }
}
