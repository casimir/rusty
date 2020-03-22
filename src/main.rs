use std::sync::mpsc::Sender;

use rusty::graphics::{Context, CoordPixel, Pixel};
use rusty::math::vec3::{Vector, Vertex};
use rusty::tracer::lights::{DirectionalLight, SphericalLight};
use rusty::tracer::objects::{Plane, Sphere};
use rusty::tracer::{Scene, Screen};

pub fn main() {
    env_logger::init();

    let mut gui = Context::new(800, 600).unwrap();
    gui.draw(raytracer);
    gui.run().unwrap();
}

fn raytracer(width: u32, height: u32, tx: Sender<CoordPixel>) {
    let mut scene = Scene::default();
    scene.add_object(Sphere {
        center: Vertex {
            x: 0.0,
            y: 0.0,
            z: -10.0,
        },
        radius: 2.0,
        base_color: "#00FFFF".parse().unwrap(),
        base_albedo: 0.8,
    });
    scene.add_object(Sphere {
        center: Vertex {
            x: -2.0,
            y: 2.0,
            z: -6.0,
        },
        radius: 2.0,
        base_color: "#FF00FF".parse().unwrap(),
        base_albedo: 0.6,
    });
    scene.add_object(Sphere {
        center: Vertex {
            x: 3.0,
            y: 0.0,
            z: -15.0,
        },
        radius: 5.0,
        base_color: "#FFFF00".parse().unwrap(),
        base_albedo: 0.7,
    });
    scene.add_object(Sphere {
        center: Vertex {
            x: 3.0,
            y: 10.0,
            z: -12.0,
        },
        radius: 3.0,
        base_color: "#CCCC00".parse().unwrap(),
        base_albedo: 0.8,
    });
    scene.add_object(Plane {
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
        base_albedo: 0.5,
    });
    scene.add_light(DirectionalLight {
        direction: Vector {
            x: -0.5,
            y: -1.5,
            z: -1.0,
        },
        base_color: "#3BEB47".parse().unwrap(),
        base_intensity: 0.6,
    });
    scene.add_light(SphericalLight {
        position: Vertex {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        base_color: "#FFFFFF".parse().unwrap(),
        base_intensity: 10000.0,
    });
    scene.add_light(SphericalLight {
        position: Vertex {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        base_color: "#EB3BE4".parse().unwrap(),
        base_intensity: 2000.0,
    });

    for (point, ray) in Screen::new(width, height) {
        let interception = match scene.trace(&ray) {
            Some(i) => i,
            None => continue,
        };
        let pixel = CoordPixel {
            x: point.0,
            y: point.1,
            pixel: Pixel::Data(scene.compute_color(&interception)),
        };
        tx.send(pixel).expect("send processed pixel");
    }
    drop(tx);
    println!("rays: {:?}", scene.stats.rays);
}
