use crate::graphics::Color;
use crate::math::vec3::{Vector, Vertex};
use crate::math::{solve_quadratic, QuadraticSolution};
use crate::tracer::Ray;

pub trait RenderableObject {
    fn color(&self) -> Color;
    fn albedo(&self) -> f32;
    fn intercept(&self, ray: &Ray) -> Option<f32>;
    fn compute_normal(&self, hitpoint: Vertex) -> Vector;
}

// TODO proc macro to impl RenderableObject
#[derive(Clone, Copy)]
pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
}

impl Object {
    pub fn color(&self) -> Color {
        match self {
            Self::Plane(o) => o.color(),
            Self::Sphere(o) => o.color(),
        }
    }

    pub fn albedo(&self) -> f32 {
        match self {
            Self::Plane(o) => o.albedo(),
            Self::Sphere(o) => o.albedo(),
        }
    }

    pub fn intercept(&self, ray: &Ray) -> Option<f32> {
        match self {
            Self::Plane(o) => o.intercept(ray),
            Self::Sphere(o) => o.intercept(ray),
        }
    }

    pub fn compute_normal(&self, hitpoint: Vertex) -> Vector {
        match self {
            Self::Plane(o) => o.compute_normal(hitpoint),
            Self::Sphere(o) => o.compute_normal(hitpoint),
        }
    }
}

impl From<Plane> for Object {
    fn from(o: Plane) -> Object {
        Object::Plane(o)
    }
}

impl From<Sphere> for Object {
    fn from(o: Sphere) -> Object {
        Object::Sphere(o)
    }
}
#[derive(Copy, Clone)]
pub struct Plane {
    pub point: Vertex,
    pub normal: Vector,
    pub base_color: Color,
    pub base_albedo: f32,
}

impl RenderableObject for Plane {
    fn color(&self) -> Color {
        self.base_color
    }

    fn albedo(&self) -> f32 {
        self.base_albedo
    }

    fn intercept(&self, ray: &Ray) -> Option<f32> {
        let denom = self.normal.dot(ray.direction);
        if denom.is_sign_positive() {
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
    pub base_albedo: f32,
}

impl RenderableObject for Sphere {
    fn color(&self) -> Color {
        self.base_color
    }

    fn albedo(&self) -> f32 {
        self.base_albedo
    }

    fn intercept(&self, ray: &Ray) -> Option<f32> {
        let l = Vector::from_vertices(self.center, ray.origin);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius.powi(2);
        match solve_quadratic(a, b, c) {
            QuadraticSolution::Two(x1, x2) => {
                if x1.is_sign_positive() && x2.is_sign_positive() {
                    Some(x2.min(x1))
                } else if x1.is_sign_positive() {
                    Some(x1)
                } else if x2.is_sign_positive() {
                    Some(x2)
                } else {
                    None
                }
            }
            QuadraticSolution::One(x) => {
                if x.is_sign_positive() {
                    Some(x)
                } else {
                    None
                }
            }
            QuadraticSolution::None => None,
        }
    }

    fn compute_normal(&self, hitpoint: Vertex) -> Vector {
        Vector::from_vertices(self.center, hitpoint).normalize()
    }
}
