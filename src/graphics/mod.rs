mod color;
mod gpu;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

pub use color::Color;
use glium::glutin::{
    self,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
};
use gpu::{GpuContextError, GpuError, PositionConverter, Vertex};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pixel {
    Data(Color),
    Blank,
}

#[derive(Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Pixel>,
    dirty: bool,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Pixel::Blank; (width * height) as usize],
            dirty: false,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Pixel {
        if x > self.width || y > self.height {
            error!("invalid coordinates: ({}, {})", x, y);
            Pixel::Blank
        } else {
            let index = (x + y * self.width) as usize;
            self.pixels[index]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) -> bool {
        if x > self.width || y > self.height {
            error!("invalid coordinates: ({}, {})", x, y);
            false
        } else {
            let index = (x + y * self.width) as usize;
            let old = self.pixels[index];
            let new = Pixel::Data(c);
            if old != new {
                self.pixels[index] = new;
                self.dirty = true;
                true
            } else {
                false
            }
        }
    }

    pub fn unset(&mut self, x: usize, y: usize) -> bool {
        if x > self.width || y > self.height {
            error!("invalid coordinates: ({}, {})", x, y);
            false
        } else {
            let index = (x + y * self.width) as usize;
            let old = self.pixels[index];
            if old != Pixel::Blank {
                self.pixels[index] = Pixel::Blank;
                self.dirty = true;
                true
            } else {
                false
            }
        }
    }

    fn as_raw(&self) -> Vec<Vertex> {
        let pc = PositionConverter::new(self.width, self.height);
        let mut data = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if let Pixel::Data(color) = self.get(x, y) {
                    data.push(Vertex {
                        position: pc.get(x, y),
                        color: [color.red, color.green, color.blue],
                    });
                }
            }
        }
        data
    }
}

#[derive(Debug)]
pub enum Error {
    GpuContext(GpuContextError),
    Gpu(GpuError),
    Export(image::ImageError),
}

impl From<GpuContextError> for Error {
    fn from(val: GpuContextError) -> Self {
        Self::GpuContext(val)
    }
}

impl From<GpuError> for Error {
    fn from(val: GpuError) -> Self {
        Self::Gpu(val)
    }
}

impl From<image::ImageError> for Error {
    fn from(val: image::ImageError) -> Error {
        Error::Export(val)
    }
}

#[derive(Debug)]
pub struct CoordPixel {
    pub pixel: Pixel,
    pub x: usize,
    pub y: usize,
}

pub type CanvasLock = Arc<RwLock<Canvas>>;

pub struct Context {
    width: usize,
    height: usize,
}

impl Context {
    pub fn new(width: usize, height: usize) -> Context {
        Context { width, height }
    }

    pub fn export(canvas: &Canvas) -> Result<String, Error> {
        use image::{ImageBuffer, Rgb};
        let img =
            ImageBuffer::from_fn(
                canvas.width as u32,
                canvas.height as u32,
                |x, y| match canvas.get(x as usize, y as usize) {
                    Pixel::Data(color) => Rgb([
                        (color.red.powf(1.0 / 2.2) * 255.0) as u8,
                        (color.green.powf(1.0 / 2.2) * 255.0) as u8,
                        (color.blue.powf(1.0 / 2.2) * 255.0) as u8,
                    ]),
                    Pixel::Blank => Rgb([0x00, 0x00, 0x00]),
                },
            );
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("get the now")
            .as_secs();
        let filename: &str = &format!("rusty_{}.png", timestamp);
        img.save(filename)?;
        log::info!("exported as: {}", filename);
        Ok(filename.into())
    }

    fn key_handler(input: KeyboardInput, canvas: &CanvasLock) -> bool {
        match input {
            KeyboardInput {
                virtual_keycode: Some(virtual_code),
                state: ElementState::Pressed,
                ..
            } => match virtual_code {
                VirtualKeyCode::Q | VirtualKeyCode::Escape => {
                    return true;
                }
                VirtualKeyCode::E => {
                    let c_lock = canvas.read().expect("read lock canvas");
                    if let Err(e) = Self::export(&*c_lock) {
                        log::error!("failed export generation: {:?}", e);
                    }
                }
                _ => {}
            },
            _ => {}
        }
        false
    }

    pub fn run(&mut self, drawer: fn(CanvasLock)) -> Result<(), Error> {
        let width = self.width;
        let height = self.height;
        let canvas = Arc::new(RwLock::new(Canvas::new(width, height)));
        let (display, pixel_program, event_loop) = gpu::init_context(width, height, "Rusty")?;

        let canvas_ = canvas.clone();
        thread::spawn(move || {
            drawer(canvas_);
        });

        event_loop.run(move |event, _, control_flow| {
            let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if Self::key_handler(input, &canvas) {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                            return;
                        }
                    }
                    _ => return,
                },
                Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }

            if canvas.read().expect("read lock canvas").dirty {
                let mut c_lock = canvas.write().expect("read lock canvas");
                let viewport = (width as f32, height as f32);
                match gpu::render_buffer(&display, c_lock.as_raw(), &pixel_program, viewport) {
                    Ok(_) => c_lock.dirty = false,
                    Err(e) => error!("paint error: {:?}", e),
                };
            }
        });
    }
}
