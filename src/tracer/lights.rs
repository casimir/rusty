use crate::graphics::Color;
use crate::math::vec3::{Vector, Vertex};

pub trait RenderableLight {
    fn color(&self) -> Color;
    fn direction_from(&self, hitpoint: Vertex) -> Vector;
    fn distance(&self, hitpoint: Vertex) -> f32;
    fn intensity(&self, hitpoint: Vertex) -> f32;
}

#[derive(Clone, Copy, Debug)]
pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match self {
            Self::Directional(l) => l.color(),
            Self::Spherical(l) => l.color(),
        }
    }

    pub fn direction_from(&self, point: Vertex) -> Vector {
        match self {
            Self::Directional(l) => l.direction_from(point),
            Self::Spherical(l) => l.direction_from(point),
        }
    }

    pub fn distance(&self, point: Vertex) -> f32 {
        match self {
            Self::Directional(l) => l.distance(point),
            Self::Spherical(l) => l.distance(point),
        }
    }

    pub fn intensity(&self, point: Vertex) -> f32 {
        match self {
            Self::Directional(l) => l.intensity(point),
            Self::Spherical(l) => l.intensity(point),
        }
    }
}

impl From<DirectionalLight> for Light {
    fn from(l: DirectionalLight) -> Light {
        Light::Directional(l)
    }
}

impl From<SphericalLight> for Light {
    fn from(l: SphericalLight) -> Light {
        Light::Spherical(l)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    pub direction: Vector,
    pub base_color: Color,
    pub base_intensity: f32,
}

impl RenderableLight for DirectionalLight {
    fn color(&self) -> Color {
        self.base_color
    }

    fn direction_from(&self, _: Vertex) -> Vector {
        -self.direction.normalize()
    }

    fn distance(&self, _: Vertex) -> f32 {
        ::std::f32::INFINITY
    }

    fn intensity(&self, _: Vertex) -> f32 {
        self.base_intensity
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SphericalLight {
    pub position: Vertex,
    pub base_color: Color,
    pub base_intensity: f32,
}

impl RenderableLight for SphericalLight {
    fn color(&self) -> Color {
        self.base_color
    }

    fn direction_from(&self, hitpoint: Vertex) -> Vector {
        Vector::from_vertices(hitpoint, self.position).normalize()
    }

    fn distance(&self, hitpoint: Vertex) -> f32 {
        Vector::from_vertices(hitpoint, self.position).norm()
    }

    fn intensity(&self, hitpoint: Vertex) -> f32 {
        let d2 = Vector::from_vertices(hitpoint, self.position)
            .norm()
            .powi(2);
        self.base_intensity / (4.0 * ::std::f32::consts::PI * d2)
    }
}
