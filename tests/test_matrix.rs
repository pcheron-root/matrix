use matrix::Matrix;

#[test]
fn test_create_matrix() {
    let _ = Matrix::new([[1, 2], [3, 4]]);
}

#[test]
fn test_print_matrix() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    m.print();
}

#[test]
fn test_shape_matrix() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.shape(), [2, 2]);
}

#[test]
fn test_is_square1_matrix() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.is_square(), true);
}

#[test]
fn test_is_square2_matrix() {
    let m = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    assert_eq!(m.is_square(), false);
}

// -----------------------------------------------------------------
// add / substrack / scale
// -----------------------------------------------------------------

#[test]
fn test_add_matrix() {
    let mut m1 = Matrix::new([[13, 4, 2], [6, 3, 9]]);
    let m2 = Matrix::new([[29, 1, 1], [6, 9, 3]]);
    m1.add(&m2);
    assert_eq!(m1.data[0][0], 42);
    assert_eq!(m1.data[1][1], 12);
    assert_eq!(m1.data[1][2], 12);

    let mut m3 = Matrix::new([[13.0, 4.0, 2.0], [6.0, 3.0, 9.0]]);
    let m4 = Matrix::new([[29.0, 1.0, 1.0], [6.0, 9.0, 3.0]]);
    m3.add(&m4);
    assert_eq!(m3.data[0][0], 42.0);
    assert_eq!(m3.data[1][1], 12.0);
    assert_eq!(m3.data[1][2], 12.0);
}

#[test]
fn test_sub_matrix() {
    let mut m1 = Matrix::new([[13, 4, 2], [6, 3, 9]]);
    let m2 = Matrix::new([[29, 1, 1], [6, 9, 3]]);
    m1.sub(&m2);
    assert_eq!(m1.data[0][0], -16);
    assert_eq!(m1.data[1][1], -6);
    assert_eq!(m1.data[1][2], 6);

    let mut m3 = Matrix::new([[13.0, 4.0, 2.0], [6.0, 3.0, 9.0]]);
    let m4 = Matrix::new([[29.0, 1.0, 1.0], [6.0, 9.0, 3.0]]);
    m3.sub(&m4);
    assert_eq!(m3.data[0][0], -16.0);
    assert_eq!(m3.data[1][1], -6.0);
    assert_eq!(m3.data[1][2], 6.0);
}

#[test]
fn test_scl_matrix() {
    let mut m1 = Matrix::new([[13, 4, 2], [6, 3, 9]]);
    let x1 = 4;
    m1.scl(x1);
    assert_eq!(m1.data[0][0], 52);
    assert_eq!(m1.data[1][1], 12);
    assert_eq!(m1.data[1][2], 36);

    let mut m2 = Matrix::new([[13.0, 4.0, 2.0], [6.0, 3.0, 9.0]]);
    let x2 = -2.0;
    m2.scl(x2);
    assert_eq!(m2.data[0][0], -26.0);
    assert_eq!(m2.data[1][1], -6.0);
    assert_eq!(m2.data[1][2], -18.0);
}
