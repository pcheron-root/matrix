use matrix::Matrix;
use matrix::Vector;

#[test]
fn test_add_operator_vector() {
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 + v2;
    assert_eq!(v3.data, [0, 0]);
}

#[test]
fn test_sub_operator_vector() {
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 - v2;
    assert_eq!(v3.data, [4, 8]);
}

#[test]
fn test_mult_operator_vector() {
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 * v2;
    assert_eq!(v3.data, [-4, -16]);
}

#[test]
fn test_div_operator_vector() {
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 / v2;
    assert_eq!(v3.data, [-1, -1]);
}

#[test]
fn test_eq_operator_vector() {
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = Vector::new([2, 4]);
    assert_eq!(v1 == v2, false);
    assert_eq!(v1 == v3, true);
}

#[test]
fn test_add_operator_matrix() {
    let m1 = Matrix::new([[2, 4], [1, 1]]);
    let m2 = Matrix::new([[-2, -4], [1, 1]]);
    let m3 = m1 + m2;
    assert_eq!(m3.data, [[0, 0], [2, 2]]);
}

#[test]
fn test_sub_operator_matrix() {
    let m1 = Matrix::new([[2, 4], [1, 1]]);
    let m2 = Matrix::new([[-2, -4], [1, 1]]);
    let m3 = m1 - m2;
    assert_eq!(m3.data, [[4, 8], [0, 0]]);
}

#[test]
fn test_mult_operator_matrix() {
    let m1 = Matrix::new([[2, 4], [1, 1]]);
    let m2 = Matrix::new([[-2, -4], [1, 1]]);
    let m3 = m1 * m2;
    assert_eq!(m3.data, [[-4, -16], [1, 1]]);
}

#[test]
fn test_div_operator_matrix() {
    let m1 = Matrix::new([[2, 4], [1, 1]]);
    let m2 = Matrix::new([[-2, -4], [1, 1]]);
    let m3 = m1 / m2;
    assert_eq!(m3.data, [[-1, -1], [1, 1]]);
}

#[test]
fn test_eq_operator_matrix() {
    let m1 = Matrix::new([[2, 4], [1, 1]]);
    let m2 = Matrix::new([[-2, -4], [1, 1]]);
    let m3 = Matrix::new([[2, 4], [1, 1]]);
    assert_eq!(m1 == m2, false);
    assert_eq!(m1 == m3, true);
}
