use std::cmp;

use graphics::Color;
use math::{Radian, QuadraticSolution, solve_quadratic};
use math::vec3::{Vector, Vertex};
use tracer::{Object, Ray};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vertex,
    pub radius: f32,
}

impl Object for Sphere {
    fn anchor(&self) -> Vertex {
        self.center
    }

    fn color(&self) -> Color {
        "#00FFFF".parse().unwrap()
    }

    fn intercept(&self, ray: &Ray) -> Option<f32> {
        let l = Vector::from_vertices(self.center, ray.origin);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius.powi(2);
        match solve_quadratic(a, b, c) {
            QuadraticSolution::Two(x1, x2) => {
                if x1 > 0.0 && x2 > 0.0 {
                    if x1 > x2 { Some(x2) } else { Some(x1) }
                } else if x1 > 0.0 {
                    Some(x1)
                } else {
                    Some(x2)
                }
            }
            QuadraticSolution::One(x) => if x > 0.0 { Some(x) } else { None },
            QuadraticSolution::None => None,
        }
    }
}
