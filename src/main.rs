extern crate rusty;
extern crate env_logger;

use std::sync::mpsc::Sender;

use rusty::graphics::{Color, Context, CoordPixel, Pixel};
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
    let objs: Vec<Sphere> = vec![Sphere {
                                     center: Vertex {
                                         x: 0.0,
                                         y: 0.0,
                                         z: -10.0,
                                     },
                                     radius: 2.0,
                                     base_color: "#00FFFF".parse().unwrap(),
                                 },
                                 Sphere {
                                     center: Vertex {
                                         x: 3.0,
                                         y: 0.0,
                                         z: -15.0,
                                     },
                                     radius: 5.0,
                                     base_color: "#FFFF00".parse().unwrap(),
                                 }];

    for (point, ray) in Screen::new(w, h) {
        let mut intercepter: Option<(Color, f32)> = None;
        for obj in objs.iter() {
            if let Some(distance) = obj.intercept(&ray) {
                if let Some((o, d)) = intercepter {
                    if distance < d {
                        intercepter = Some((obj.color(), distance));
                    }
                } else {
                    intercepter = Some((obj.color(), distance));
                }
            }
        }
        if let Some((color, distance)) = intercepter {
            let pixel = CoordPixel {
                x: point.0,
                y: point.1,
                pixel: Pixel::Data(color),
            };
            tx.send(pixel);
        }
    }
    drop(tx);
    println!("It's over S");
}
