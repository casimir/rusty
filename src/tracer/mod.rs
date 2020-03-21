pub mod objects;

use std::collections::HashMap;

use crate::graphics::Color;
use crate::math::vec3::{Vector, Vertex};
use rand::Rng;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RayKind {
    Primary,
    Shadow,
    Reflection,
    Refraction,
}

pub struct Ray {
    pub kind: RayKind,
    pub origin: Vertex,
    pub direction: Vector,
}

pub type Tracer = dyn Iterator<Item = Ray>;

pub struct Screen {
    width: f32,
    height: f32,
    cursor: usize,
    points: Vec<(f32, f32)>,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        let mut points = Vec::new();
        for y in 0..height {
            for x in 0..width {
                points.push((x as f32, y as f32))
            }
        }
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut points);
        Screen {
            width: width as f32,
            height: height as f32,
            cursor: 0,
            points: points,
        }
    }
}

impl Iterator for Screen {
    type Item = ((u32, u32), Ray);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.points.len() {
            return None;
        }

        let point = self.points[self.cursor];
        self.cursor += 1;
        let origin = Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let screen_vec = Vertex {
            x: (((point.0 + 0.5) / self.width) * 2.0 - 1.0) * self.width / self.height,
            y: 1.0 - ((point.1 + 0.5) / self.height) * 2.0,
            z: -1.0,
        };
        Some((
            (point.0 as u32, point.1 as u32),
            Ray {
                kind: RayKind::Primary,
                origin: origin,
                direction: Vector::from_vertices(origin, screen_vec),
            },
        ))
    }
}

pub trait Object {
    fn color(&self) -> Color;
    fn albedo(&self) -> f32;
    fn intercept(&self, ray: &Ray) -> Option<f32>;
    fn compute_normal(&self, hitpoint: Vertex) -> Vector;
}

pub struct Interception<'a> {
    pub object: &'a Box<dyn Object>,
    pub distance: f32,
    pub hitpoint: Vertex,
}

impl<'a> Interception<'a> {
    pub fn new<'b>(object: &'b Box<dyn Object>, ray: &Ray, distance: f32) -> Interception<'b> {
        Interception {
            object: object,
            distance: distance,
            hitpoint: Vertex {
                x: ray.origin.x + ray.direction.x * distance,
                y: ray.origin.y + ray.direction.y * distance,
                z: ray.origin.z + ray.direction.z * distance,
            },
        }
    }
}

pub struct Light {
    pub direction: Vector,
    pub color: Color,
    pub intensity: f32,
}

pub struct Statistics {
    pub rays: HashMap<RayKind, usize>,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            rays: HashMap::new(),
        }
    }

    pub fn count_ray(&mut self, ray: &Ray) {
        let counter = self.rays.entry(ray.kind.clone()).or_insert(0);
        *counter += 1;
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<Light>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn trace(&self, ray: &Ray) -> Option<Interception> {
        self.objects
            .iter()
            .filter_map(|o| o.intercept(ray).map(|d| Interception::new(o, ray, d)))
            .min_by(|d1, d2| d1.distance.partial_cmp(&d2.distance).unwrap())
    }
}
