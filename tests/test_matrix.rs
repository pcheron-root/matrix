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
