use std::sync::mpsc::Sender;

use rusty::graphics::{Context, CoordPixel, Pixel};
use rusty::math::vec3::{Vector, Vertex};
use rusty::tracer::objects::{Plane, Sphere};
use rusty::tracer::{Light, Ray, RayKind, Scene, Screen};

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
    scene.lights.push(Box::new(Light {
        direction: Vector {
            x: -0.5,
            y: -1.5,
            z: -1.0,
        },
        color: "#FFFFFF".parse().unwrap(),
        intensity: 0.9,
    }));
    scene.lights.push(Box::new(Light {
        direction: Vector {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        color: "#FFA50C".parse().unwrap(),
        intensity: 0.4,
    }));

    let lights = scene.lights.clone();
    for (point, ray) in Screen::new(width, height) {
        let interception = match scene.trace(&ray) {
            Some(i) => i,
            None => continue,
        };
        let normal = interception.object.compute_normal(interception.hitpoint);
        let mut color = Default::default();
        for light in &lights {
            let light_direction = -light.direction.normalize();
            let shadow_origin = Vertex {
                x: interception.hitpoint.x + normal.x * 1e-5,
                y: interception.hitpoint.y + normal.y * 1e-5,
                z: interception.hitpoint.z + normal.z * 1e-5,
            };
            let shadow_ray = Ray {
                origin: shadow_origin,
                direction: light_direction,
                kind: RayKind::Shadow,
            };
            if scene.trace(&shadow_ray).is_none() {
                let light_power = normal.dot(light_direction).max(0.0)
                    * light.intensity
                    * interception.object.albedo();
                color += interception.object.color() * light.color * light_power;
            }
        }
        let pixel = CoordPixel {
            x: point.0,
            y: point.1,
            pixel: Pixel::Data(color),
        };
        tx.send(pixel).expect("send processed pixel");
    }
    drop(tx);
    println!("rays: {:?}", scene.stats.rays);
}
