use std::num::Float;
use std::ops::{Add, Div, Mul, Sub};

pub struct Vector { 
    pub x: f32,
    pub y: f32,
    pub z: f32 
}

impl Vector {
    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        match self.norm() {
            0.0 => Vector { x: 0.0, y: 0.0, z: 0.0 },
            n => Vector { x: self.x / n, y: self.y / n, z: self.z / n }
        }
    }

    pub fn cross(&self, v: Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }

    pub fn dot(&self, v: Vector) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, _rhs: f32) -> Vector {
        Vector {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, _rhs: f32) -> Vector {
        Vector {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}
