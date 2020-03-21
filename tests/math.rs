extern crate rusty;

use rusty::math::*;

#[test]
fn quadratic() {
    assert_eq!(solve_quadratic(2.0, -4.0, 7.0), QuadraticSolution::None);
    assert_eq!(
        solve_quadratic(-3.0, -24.0, -48.0),
        QuadraticSolution::One(-4.0)
    );
    assert_eq!(
        solve_quadratic(4.0, -4.0, -24.0),
        QuadraticSolution::Two(3.0, -2.0)
    );
}

use rusty::math::vec3::*;

#[test]
fn vector_from_vertices() {
    let v1 = Vertex {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = Vertex {
        x: 10.0,
        y: 0.0,
        z: 100.0,
    };
    let expected = Vector {
        x: 9.0,
        y: -2.0,
        z: 97.0,
    };
    assert_eq!(Vector::from_vertices(v1, v2), expected);
}
