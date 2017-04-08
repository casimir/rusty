use graphics::Color;
use math::{QuadraticSolution, solve_quadratic};
use math::vec3::{Vector, Vertex};
use tracer::{Object, Ray};

#[derive(Copy, Clone)]
pub struct Plane {
    pub point: Vertex,
    pub normal: Vector,
    pub base_color: Color,
}

impl Object for Plane {
    fn color(&self) -> Color {
        self.base_color
    }

    fn intercept(&self, ray: &Ray) -> Option<f32> {
        let denom = self.normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = Vector::from_vertices(ray.origin, self.point);
            let distance = v.dot(self.normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn compute_normal(&self, _: Vertex) -> Vector {
        -self.normal
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vertex,
    pub radius: f32,
    pub base_color: Color,
}

impl Object for Sphere {
    fn color(&self) -> Color {
        self.base_color
    }

    fn intercept(&self, ray: &Ray) -> Option<f32> {
        let l = Vector::from_vertices(self.center, ray.origin);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius.powi(2);
        match solve_quadratic(a, b, c) {
            QuadraticSolution::Two(x1, x2) => {
                if x1 > 1e-6 && x2 > 1e-6 {
                    Some(x2.min(x1))
                } else if x1 > 1e-6 {
                    Some(x1)
                } else if x2 > 1e-6 {
                    Some(x2)
                } else {
                    None
                }
            }
            QuadraticSolution::One(x) => if x > 0.0 { Some(x) } else { None },
            QuadraticSolution::None => None,
        }
    }

    fn compute_normal(&self, hitpoint: Vertex) -> Vector {
        Vector::from_vertices(self.center, hitpoint).normalize()
    }
}
