extern crate rand;

pub mod objects;

use self::rand::Rng;

use graphics::Color;
use math::Radian;
use math::vec3::{Vector, Vertex};

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

pub type Tracer = Iterator<Item = Ray>;

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
        Some(((point.0 as u32, point.1 as u32),
              Ray {
                  kind: RayKind::Primary,
                  origin: origin,
                  direction: Vector::from_vertices(origin, screen_vec),
              }))
    }
}

pub trait Object {
    fn anchor(&self) -> Vertex;
    fn color(&self) -> Color;
    fn intercept(&self, ray: &Ray) -> Option<f32>;
}
