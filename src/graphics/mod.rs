extern crate image;
extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color::RGB;
use self::sdl2::render::Renderer;
use self::sdl2::rect::Point;
use std::str::FromStr;
use std::sync::mpsc;
use std::time::Duration;
use std::thread;

#[derive(Debug, PartialEq)]
pub enum ColorError {
    InvalidColorError,
    ConversionError(::std::num::ParseIntError),
}

impl From<::std::num::ParseIntError> for ColorError {
    fn from(val: ::std::num::ParseIntError) -> ColorError {
        ColorError::ConversionError(val)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(src: &str) -> Result<Color, ColorError> {
        if !src.starts_with('#') {
            return Err(ColorError::InvalidColorError);
        }
        match src.chars().count() {
            7usize => {
                Ok(Color {
                    r: u8::from_str_radix(&src[1..3], 16)?,
                    g: u8::from_str_radix(&src[3..5], 16)?,
                    b: u8::from_str_radix(&src[5..], 16)?,
                    a: 255,
                })
            }
            9usize => {
                Ok(Color {
                    r: u8::from_str_radix(&src[1..3], 16)?,
                    g: u8::from_str_radix(&src[3..5], 16)?,
                    b: u8::from_str_radix(&src[5..7], 16)?,
                    a: u8::from_str_radix(&src[7..], 16)?,
                })
            }
            _ => Err(ColorError::InvalidColorError),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width: width,
            height: height,
            pixels: vec![Pixel::Blank; (width * height) as usize],
            dirty: true,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Pixel {
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
            let old = self.pixels[index];
            let new = Pixel::Data(c);
            if old != new {
                self.pixels[index] = new;
                self.dirty = true;
            }
        }
    }

    pub fn unset(&mut self, x: u32, y: u32) {
        if x > self.width || y > self.height {
            error!("invalid coordinates: ({}, {})", x, y);
        } else {
            let index = (x + y * self.width) as usize;
            let old = self.pixels[index];
            if old != Pixel::Blank {
                self.pixels[index] = Pixel::Blank;
                self.dirty = true;
            }
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

#[derive(Debug)]
pub struct CoordPixel {
    pub pixel: Pixel,
    pub x: u32,
    pub y: u32,
}

pub struct Context {
    context: sdl2::Sdl,
    canvas: Canvas,
    renderer: Renderer<'static>,
    drawer_rx: Option<mpsc::Receiver<CoordPixel>>,
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
            canvas: Canvas::new(width, height),
            renderer: renderer,
            drawer_rx: None,
        })
    }

    pub fn paint(&mut self) {
        if let Some(rx) = self.drawer_rx.take() {
            for point in rx.try_iter() {
                match point.pixel {
                    Pixel::Data(color) => self.canvas.set(point.x, point.y, color),
                    Pixel::Blank => self.canvas.unset(point.x, point.y),
                }
            }
            self.drawer_rx = Some(rx);
        }
        if self.canvas.dirty {
            let rend = &mut self.renderer;
            for x in 0..self.canvas.width {
                for y in 0..self.canvas.height {
                    if let Pixel::Data(color) = self.canvas.get(x, y) {
                        rend.set_draw_color(RGB(color.r, color.g, color.b));
                        rend.draw_point(Point::new(x as i32, y as i32));
                    };
                }
            }
            self.canvas.dirty = false;
            rend.present();
            // XXX show metrics
        }
    }

    pub fn draw(&mut self, drawer: fn(u32, u32, mpsc::Sender<CoordPixel>)) {
        if let Some(rx) = self.drawer_rx.take() {
            drop(rx);
        }
        let (tx, rx) = mpsc::channel();
        self.drawer_rx = Some(rx);
        let width = self.canvas.width;
        let height = self.canvas.height;
        thread::spawn(move || { drawer(width, height, tx); });
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
            thread::sleep(Duration::from_millis(17));
        }
        if let Some(rx) = self.drawer_rx.take() {
            drop(rx);
        }
        Ok(())
    }
}
