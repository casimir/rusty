use math::vec3::{Angle, Vector, Vertex};

pub trait Object {
    fn anchor(&self) -> Vertex;
    fn intercept(&self, vector: &Vector) -> Option<Angle>;
}

#[derive(Copy, Clone)]
pub struct Plane {
    upper_left: Vertex,
    lower_right: Vertex,
}

impl Object for Plane {
    fn anchor(&self) -> Vertex {
        Vertex {
            x: (self.upper_left.x - self.lower_right.x).abs() / 2.0,
            y: (self.upper_left.y - self.lower_right.y).abs() / 2.0,
            z: (self.upper_left.z - self.lower_right.z).abs() / 2.0,
        }
    }

    fn intercept(&self, vector: &Vector) -> Option<Angle> {
        None
    }
}

#[derive(Copy, Clone)]
pub struct Ball {
    center: Vertex,
}

impl Object for Ball {
    fn anchor(&self) -> Vertex {
        self.center
    }

    fn intercept(&self, vector: &Vector) -> Option<Angle> {
        None
    }
}
