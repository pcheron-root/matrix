use matrix::linear_interpolation::lerp;
use matrix::{Matrix, Vector};

/////////////////////////////////////////////////////
/// Exercice 02 - Linear interpolation
/////////////////////////////////////////////////////

#[test]
fn test_linear_interpolation_float() {
    let x = 2.0;
    let y = 8.0;
    assert_eq!(lerp(x, y, 0.5), 5.0);
    assert_eq!(lerp(x, y, 0.2), 3.2);
}

#[test]
fn test_linear_interpolation_int() {
    let x = 2;
    let y = 8;
    assert_eq!(lerp(x, y, 2), 14);
    assert_eq!(lerp(x, y, 3), 20);
    assert_eq!(lerp(x, y, -1), -4);
}

#[test]
fn test_linear_interpolation_vector() {
    let v1 = Vector::new([2.0, 2.0]);
    let v2 = Vector::new([4.0, 4.0]);
    let v3 = Vector::new([3.0, 3.0]);
    assert_eq!(lerp(v1.clone(), v2.clone(), 0.5), v3);
    let v4 = Vector::new([6.0, 6.0]);
    assert_eq!(lerp(v1.clone(), v2.clone(), 2.0), v4);
    let v5 = Vector::new([0.0, 0.0]);
    assert_eq!(lerp(v1, v2, -1.0), v5);
}

#[test]
fn test_linear_interpolation_vector_2() {
    let v1 = Vector::new([2.0, 1.0]);
    let v2 = Vector::new([4.0, 2.0]);
    let v3 = Vector::new([2.6, 1.3]);
    assert_eq!(lerp(v1.clone(), v2.clone(), 0.3), v3);
}

#[test]
fn test_linear_interpolation_matrix() {
    let m1 = Matrix::new([[2.0, 2.0], [4.0, 2.0]]);
    let m2 = Matrix::new([[4.0, 4.0], [3.0, 1.0]]);
    let m3 = Matrix::new([[3.0, 3.0], [3.5, 1.5]]);

    assert_eq!(lerp(m1.clone(), m2.clone(), 0.5), m3);
}

#[test]
fn test_linear_interpolation_matrix_2() {
    let m1 = Matrix::new([[2.0, 1.0], [3.0, 4.0]]);
    let m2 = Matrix::new([[20.0, 10.0], [30.0, 40.0]]);
    let m3 = Matrix::new([[11.0, 5.5], [16.5,22.]]);

    assert_eq!(lerp(m1.clone(), m2.clone(), 0.5), m3);
}
