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
        QuadraticSolution::Two(
            (-1.0 * b + discriminant.sqrt()) / (2.0 * a),
            (-1.0 * b - discriminant.sqrt()) / (2.0 * a),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_two() {
        assert_eq!(
            solve_quadratic(1.0, 5.0, 6.0),
            QuadraticSolution::Two(-2.0, -3.0)
        );
        assert_eq!(
            solve_quadratic(1.0, 7.0, 10.0),
            QuadraticSolution::Two(-2.0, -5.0)
        );
        assert_eq!(
            solve_quadratic(15.0, 5.0, 0.0),
            QuadraticSolution::Two(0.0, -1.0 / 3.0)
        );
    }

    #[test]
    fn quadratic_one() {
        assert_eq!(
            solve_quadratic(1.0, 8.0, 16.0),
            QuadraticSolution::One(-4.0)
        );
    }

    #[test]
    fn quadratic_zero() {
        assert_eq!(solve_quadratic(1.0, 2.0, 3.0), QuadraticSolution::None);
        assert_eq!(solve_quadratic(1.0, 8.0, 17.0), QuadraticSolution::None);
    }
}
