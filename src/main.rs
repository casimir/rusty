extern crate rusty;
extern crate env_logger;

use std::sync::mpsc::Sender;

use rusty::graphics::{Context, CoordPixel, Pixel};
use rusty::math::vec3::Vertex;
use rusty::tracer::{Object, Screen};
use rusty::tracer::objects::Sphere;

pub fn main() {
    env_logger::init().unwrap();

    let mut gui = Context::new(800, 600).unwrap();
    gui.draw(raytracer);
    gui.run().unwrap();
}

fn raytracer(w: u32, h: u32, tx: Sender<CoordPixel>) {
    let obj = Sphere {
        center: Vertex {
            x: 0.0,
            y: 0.0,
            z: -10.0,
        },
        radius: 2.0,
    };

    for (point, ray) in Screen::new(w, h) {
        if let Some(distance) = obj.intercept(&ray) {
            let pixel = CoordPixel {
                x: point.0,
                y: point.1,
                pixel: Pixel::Data(obj.color()),
            };
            tx.send(pixel);
        }
    }
    drop(tx);
    println!("It's over S");
}
