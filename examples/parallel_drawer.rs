extern crate rusty;

use std::sync::mpsc::Sender;
use std::thread;

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
        thread::spawn(move || {
            fill_part(w, h, start, end, tx);
        });
    }
}

fn fill_part(w: u32, h: u32, start: u32, end: u32, tx: Sender<CoordPixel>) {
    println!("worker: {:3} -> {:3}", start, end);
    for y in start..end {
        for x in 0..w {
            let blue = (((x * y) as f32 / (h.pow(2) + w.pow(2)) as f32).sqrt()) as f32;
            let pixel = CoordPixel {
                x,
                y,
                pixel: Pixel::Data(Color {
                    red: (x as f32 / w as f32),
                    green: (y as f32 / h as f32),
                    blue,
                }),
            };
            std::thread::sleep(std::time::Duration::from_millis(1));
            tx.send(pixel).expect("send pixel");
        }
    }
}
