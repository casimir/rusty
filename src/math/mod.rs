use std::convert::From;

pub mod vec3;

pub struct Degree(f32);
pub struct Radian(f32);

impl From<Radian> for Degree {
    fn from(angle: Radian) -> Degree {
        Degree(angle.0.to_degrees())
    }
}

impl From<Degree> for Radian {
    fn from(angle: Degree) -> Radian {
        Radian(angle.0.to_radians())
    }
}

#[derive(Debug, PartialEq)]
pub enum QuadraticSolution {
    None,
    One(f32),
    Two(f32, f32),
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> QuadraticSolution {
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        QuadraticSolution::None
    } else if discriminant == 0.0 {
        QuadraticSolution::One(-0.5 * b / a)
    } else {
        QuadraticSolution::Two((-1.0 * b + discriminant.sqrt()) / (2.0 * a),
                               (-1.0 * b - discriminant.sqrt()) / (2.0 * a))
    }
}
