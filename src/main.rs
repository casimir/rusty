extern crate rusty;
extern crate env_logger;

use std::sync::mpsc::Sender;

use rusty::graphics::{Color, Context, CoordPixel, Pixel};
use rusty::math::vec3::{Vector, Vertex};
use rusty::tracer::{Light, Object, Screen};
use rusty::tracer::objects::{Plane, Sphere};

pub fn main() {
    env_logger::init().unwrap();

    let mut gui = Context::new(800, 600).unwrap();
    gui.draw(raytracer);
    gui.run().unwrap();
}

fn raytracer(w: u32, h: u32, tx: Sender<CoordPixel>) {
    let lights: Vec<Light> = vec![];
    let mut objs: Vec<Box<Object>> = Vec::new();
    objs.push(Box::new(Sphere {
                           center: Vertex {
                               x: 0.0,
                               y: 0.0,
                               z: -10.0,
                           },
                           radius: 2.0,
                           base_color: "#00FFFF".parse().unwrap(),
                       }));
    objs.push(Box::new(Sphere {
                           center: Vertex {
                               x: -2.0,
                               y: 2.0,
                               z: -6.0,
                           },
                           radius: 2.0,
                           base_color: "#FF00FF".parse().unwrap(),
                       }));
    objs.push(Box::new(Sphere {
                           center: Vertex {
                               x: 3.0,
                               y: 0.0,
                               z: -15.0,
                           },
                           radius: 5.0,
                           base_color: "#FFFF00".parse().unwrap(),
                       }));
    objs.push(Box::new(Plane {
                           point: Vertex {
                               x: 0.0,
                               y: -5.0,
                               z: 0.0,
                           },
                           normal: Vector {
                               x: 0.0,
                               y: -1.0,
                               z: 0.0,
                           },
                           base_color: "#CCCCCC".parse().unwrap(),
                       }));

    for (point, ray) in Screen::new(w, h) {
        let mut intercepter: Option<(Color, f32)> = None;
        for obj in objs.iter() {
            if let Some(distance) = obj.intercept(&ray) {
                if let Some((_, d)) = intercepter {
                    if distance < d {
                        intercepter = Some((obj.color(), distance));
                    }
                } else {
                    intercepter = Some((obj.color(), distance));
                }
            }
        }
        if let Some((c, _)) = intercepter {
            let pixel = CoordPixel {
                x: point.0,
                y: point.1,
                pixel: Pixel::Data(c),
            };
            tx.send(pixel);
        }
    }
    drop(tx);
    println!("It's over S");
}
