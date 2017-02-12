extern crate env_logger;
#[macro_use]
extern crate log;

mod graphics;
mod math;

use graphics::Color;

pub fn main() {
    env_logger::init().unwrap();

    let mut gui = graphics::Context::new(800, 600).unwrap();
    colormap(&mut gui.canvas);
    gui.run().unwrap();
}

fn colormap(canvas: &mut graphics::Canvas) {
    let h = canvas.height;
    let w = canvas.width;
    for y in 0..canvas.height {
        for x in 0..canvas.width {
            canvas.set(x,
                       y,
                       Color {
                           r: (x as f64 / w as f64 * 255.0) as u8,
                           g: (y as f64 / h as f64 * 255.0) as u8,
                           b: (((x * y) as f64 / (h.pow(2) + w.pow(2)) as f64).sqrt() * 255.0) as
                              u8,
                           a: 255,
                       })
        }
    }
}
