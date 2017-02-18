extern crate rusty;

use std::thread;
use std::sync::mpsc::Sender;

use rusty::graphics::{Color, Context, CoordPixel, Pixel};

const NUM_THREADS: u32 = 4;

pub fn main() {
    let mut gui = Context::new(800, 600).unwrap();
    gui.draw(drawer);
    gui.run().unwrap();
}

fn drawer(w: u32, h: u32, tx: Sender<CoordPixel>) {
    for i in 0..NUM_THREADS {
        let step = h / NUM_THREADS;
        let (start, end) = (step * i, step * (i + 1));
        let tx = tx.clone();
        thread::spawn(move || { fill_part(w, h, start, end, tx); });
    }
}

fn fill_part(w: u32, h: u32, start: u32, end: u32, tx: Sender<CoordPixel>) {
    println!("worker: {:3} -> {:3}", start, end);
    for y in start..end {
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
}
