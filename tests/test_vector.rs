use matrix::Vector;

#[test]
fn test_create_vector() {
    let _ = Vector::new([42]);
}

#[test]
fn test_print_vector() {
    let v = Vector::new([42]);
    v.print();
}

// -----------------------------------------------------------------
// Exercice 00 - Add / substract / scale
//  (look at test_matrix.rs file for matrix part of ex00)
// -----------------------------------------------------------------

#[test]
fn test_add_vector() {
    let mut v1 = Vector::new([13, 4, 2]);
    let v2 = Vector::new([29, 1, 1]);
    v1.add(&v2);
    assert_eq!(v1.data[0], 42);
    assert_eq!(v1.data[1], 5);
    assert_eq!(v1.data[2], 3);

    let mut v3 = Vector::new([13.0, 4.0, 2.0]);
    let v4 = Vector::new([29.0, 1.0, 1.0]);
    v3.add(&v4);
    assert_eq!(v3.data[0], 42.0);
    assert_eq!(v3.data[1], 5.0);
    assert_eq!(v3.data[2], 3.0);
}

#[test]
fn test_add_vector_2() {
    let mut v1 = Vector::new([2, 3]);
    let v2 = Vector::new([5, 7]);
    v1.add(&v2);
    assert_eq!(v1.data[0], 7);
    assert_eq!(v1.data[1], 10);

}

#[test]
fn test_sub_vector() {
    let mut v1 = Vector::new([13, 4, 2]);
    let v2 = Vector::new([12, 1, 1]);
    v1.sub(&v2);
    assert_eq!(v1.data[0], 1);
    assert_eq!(v1.data[1], 3);
    assert_eq!(v1.data[2], 1);

    let mut v3 = Vector::new([4.0, 64.2, 2.4]);
    let v4 = Vector::new([12.0, 1.4, 1.26]);
    v3.sub(&v4);
    assert_eq!(v3.data[0], -8.0);
    assert_eq!(v3.data[1], 62.800000000000004);
    assert_eq!(v3.data[2], 1.14);
}

#[test]
fn test_sub_vector_2() {
    let mut v1 = Vector::new([2, 3]);
    let v2 = Vector::new([5, 7]);
    v1.sub(&v2);
    assert_eq!(v1.data[0], -3);
    assert_eq!(v1.data[1], -4);
}

#[test]
fn test_scl_vector() {
    let mut v1 = Vector::new([13, 4, 2]);
    let x = 2;
    v1.scl(x);
    assert_eq!(v1.data[0], 26);
    assert_eq!(v1.data[1], 8);
    assert_eq!(v1.data[2], 4);

    let mut v2 = Vector::new([5.2, 0.5, 1.12]);
    let x = 2.0;
    v2.scl(x);
    assert_eq!(v2.data[0], 10.4);
    assert_eq!(v2.data[1], 1.0);
    assert_eq!(v2.data[2], 2.24);
}

#[test]
fn test_scl_vector_2() {
    let mut v1 = Vector::new([2., 3.]);
    let x = 2.;
    v1.scl(x);
    assert_eq!(v1.data[0], 4.);
    assert_eq!(v1.data[1], 6.);
}

// -----------------------------------------------------------------
// Exercice 01 - linear combination
// -----------------------------------------------------------------

use matrix::structures::vector;

#[test]
fn test_linear_comb_matrix() {
    let v1 = Vector::new([5.0, 2.0, 13.0]);
    let v2 = Vector::new([1.0, 4.0, 3.0]);
    let v3 = Vector::new([6.0, 10.0, 2.0]);

    let x1 = 0.5;
    let x2 = 0.2;
    let x3 = 0.8;

    let v4 = vector::linear_combination(&[v1, v2, v3], &[x1, x2, x3]);
    assert_eq!(v4.data[0], 7.500000000000001);
    assert_eq!(v4.data[1], 9.8);
    assert_eq!(v4.data[2], 8.7);
    assert!((v4.data[0] - 7.5f32).abs() < 1e-6);
}

#[test]
fn test_linear_comb_matrix_2() {
    let v1 = Vector::new([1.0, 0.0, 0.0]);
    let v2 = Vector::new([0.0, 1.0, 0.0]);
    let v3 = Vector::new([0.0, 0.0, 1.0]);

    let x1 = 10.;
    let x2 = -2.;
    let x3 = 0.5;

    let v4 = vector::linear_combination(&[v1, v2, v3], &[x1, x2, x3]);
    assert_eq!(v4.data[0], 10.);
    assert_eq!(v4.data[1], -2.);
    assert_eq!(v4.data[2], 0.5);

    let v1 = Vector::new([1.0, 2.0, 3.0]);
    let v2 = Vector::new([0.0, 10.0, -100.0]);

    let x1 = 10.;
    let x2 = -2.;

    let v4 = vector::linear_combination(&[v1, v2], &[x1, x2]);
    assert_eq!(v4.data[0], 10.);
    assert_eq!(v4.data[1], 0.);
    assert_eq!(v4.data[2], 230.);
}
