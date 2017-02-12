extern crate image;
extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color::RGB;
use self::sdl2::render::Renderer;
use self::sdl2::rect::Point;
use std::time::Duration;

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
    pub width: u32,
    pub height: u32,
    pixels: Vec<Pixel>,
    dirty: bool,
}

impl Canvas {
    fn get(&self, x: u32, y: u32) -> Pixel {
        if x > self.width || y > self.height {
            error!("invalid coordinates: ({}, {})", x, y);
            Pixel::Blank
        } else {
            let index = (x + y * self.width) as usize;
            self.pixels[index]
        }
    }

    pub fn set(&mut self, x: u32, y: u32, c: Color) {
        if x > self.width || y > self.height {
            error!("invalid coordinates: ({}, {})", x, y);
        } else {
            let index = (x + y * self.width) as usize;
            self.pixels[index] = Pixel::Data(c);
            self.dirty = true;
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Creation(sdl2::video::WindowBuildError),
    InvalidData(sdl2::IntegerOrSdlError),
    Export(::std::io::Error),
}

impl From<sdl2::video::WindowBuildError> for Error {
    fn from(val: sdl2::video::WindowBuildError) -> Error {
        Error::Creation(val)
    }
}

impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(val: sdl2::IntegerOrSdlError) -> Error {
        Error::InvalidData(val)
    }
}

impl From<::std::io::Error> for Error {
    fn from(val: ::std::io::Error) -> Error {
        Error::Export(val)
    }
}

pub struct Context {
    pub canvas: Canvas,
    pub context: sdl2::Sdl,
    renderer: Renderer<'static>,
}

impl Context {
    pub fn new(width: u32, height: u32) -> Result<Context, Error> {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let window = video.window("Rusty", width, height)
            .position_centered()
            .opengl()
            .build()?;
        let renderer = window.renderer().build()?;

        Ok(Context {
            context: sdl_context,
            renderer: renderer,
            canvas: Canvas {
                width: width,
                height: height,
                pixels: vec![Pixel::Blank; (width * height) as usize],
                dirty: false,
            },
        })
    }

    pub fn paint(&mut self) {
        if self.canvas.dirty {
            let ref mut rend = self.renderer;
            for x in 0..self.canvas.width {
                for y in 0..self.canvas.height {
                    match self.canvas.get(x, y) {
                        Pixel::Data(color) => {
                            rend.set_draw_color(RGB(color.r, color.g, color.b));
                            rend.draw_point(Point::new(x as i32, y as i32));
                        }
                        Pixel::Blank => {}
                    };
                }
            }
            self.canvas.dirty = false;
            rend.present();
            // XXX show metrics
        }
    }

    pub fn export(&self) -> Result<String, Error> {
        use self::image::{ImageBuffer, Rgba};
        let img = ImageBuffer::from_fn(self.canvas.width,
                                       self.canvas.height,
                                       |x, y| match self.canvas.get(x, y) {
                                           Pixel::Data(color) => {
                                               Rgba { data: [color.r, color.g, color.b, color.a] }
                                           }
                                           Pixel::Blank => Rgba { data: [0, 0, 0, 0] },
                                       });
        let filename = "export.png";
        img.save(filename)?;
        println!("exported as: {}", filename);
        Ok(filename.into())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut event_pump = self.context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Q), .. } => break 'running,
                    Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                        self.export().unwrap();
                    }
                    _ => {}
                }
            }

            self.paint();
            ::std::thread::sleep(Duration::from_millis(17));
        }
        Ok(())
    }
}
