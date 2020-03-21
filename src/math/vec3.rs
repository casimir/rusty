use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn from_vertices(v1: Vertex, v2: Vertex) -> Vector {
        Vector {
            x: v2.x - v1.x,
            y: v2.y - v1.y,
            z: v2.z - v1.z,
        }
    }

    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        match self.norm() {
            0.0 => Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            n => Vector {
                x: self.x / n,
                y: self.y / n,
                z: self.z / n,
            },
        }
    }

    pub fn cross(&self, v: Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn dot(&self, v: Vector) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
