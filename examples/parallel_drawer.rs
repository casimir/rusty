extern crate rusty;

use std::thread;

use rusty::graphics::{CanvasLock, Color, Context};

const NUM_THREADS: usize = 4;

fn main() -> Result<(), rusty::graphics::Error> {
    let mut gui = Context::new(800, 600);
    gui.run(drawer)
}

fn drawer(canvas: CanvasLock) {
    let (width, height) = {
        let c = canvas.read().expect("read lock canvas");
        (c.width, c.height)
    };
    for i in 0..NUM_THREADS {
        let step = height / NUM_THREADS;
        let (start, end) = (step * i, step * (i + 1));
        let canvas_ = canvas.clone();
        thread::spawn(move || {
            fill_part(width, height, start, end, canvas_);
        });
    }
}

fn fill_part(w: usize, h: usize, start: usize, end: usize, canvas: CanvasLock) {
    println!("worker: {:3} -> {:3}", start, end);
    for y in start..end {
        for x in 0..w {
            let blue = (((x * y) as f32 / (h.pow(2) + w.pow(2)) as f32).sqrt()) as f32;
            let color = Color {
                red: (x as f32 / w as f32),
                green: (y as f32 / h as f32),
                blue,
            };
            canvas.write().expect("write lock canvas").set(x, y, color);
        }
    }
}
