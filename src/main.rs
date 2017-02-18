extern crate rusty;
extern crate env_logger;

use std::sync::mpsc::Sender;

mod math;

use rusty::graphics::{Color, CoordPixel, Pixel};

pub fn main() {
    env_logger::init().unwrap();

    let mut gui = rusty::graphics::Context::new(800, 600).unwrap();
    gui.draw(colormap);
    gui.run().unwrap();
}

fn colormap(w: u32, h: u32, tx: Sender<CoordPixel>) {
    for y in 0..h {
        for x in 0..w {
            let pixel = CoordPixel {
                x: x,
                y: y,
                pixel: Pixel::Data(Color {
                    r: (x as f64 / w as f64 * 255.0) as u8,
                    g: (y as f64 / h as f64 * 255.0) as u8,
                    b: (((x * y) as f64 / (h.pow(2) + w.pow(2)) as f64).sqrt() * 255.0) as u8,
                    a: 255,
                }),
            };
            std::thread::sleep(std::time::Duration::from_millis(1));
            tx.send(pixel);
        }
    }
    drop(tx);
}