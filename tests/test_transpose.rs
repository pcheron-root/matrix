
use matrix::Matrix;

// -----------------------------------------------------------------
// Exercice 08 - Transpose
// -----------------------------------------------------------------

#[test]
fn test_transpose_2x2() {
    let mat = Matrix::new([[1.0, 2.0],
                           [3.0, 4.0]]);
    let transposed = mat.transpose();
    assert_eq!(transposed, Matrix::new([[1.0, 3.0],
                                        [2.0, 4.0]]));
}

    #[test]
fn test_transpose_2x3() {
    let mat = Matrix::new([[1.0, 2.0, 3.0],
                           [4.0, 5.0, 6.0]]);
    let transposed = mat.transpose();
    assert_eq!(transposed, Matrix::new([[1.0, 4.0],
                                        [2.0, 5.0],
                                        [3.0, 6.0]]));
}

    #[test]
fn test_transpose_3x2() {
    let mat = Matrix::new([[7.0, 8.0],
                           [9.0, 10.0],
                           [11.0, 12.0]]);
    let transposed = mat.transpose();
    assert_eq!(transposed, Matrix::new([[7.0, 9.0, 11.0],
                                        [8.0, 10.0, 12.0]]));
}

    #[test]
fn test_transpose_identity_3x3() {
    let mat = Matrix::new([[1.0, 0.0, 0.0],
                           [0.0, 1.0, 0.0],
                           [0.0, 0.0, 1.0]]);
    let transposed = mat.transpose();
    assert_eq!(transposed, mat);
}

#[test]
fn test_transpose_single_element() {
    let mat = Matrix::new([[42.0]]);
    let transposed = mat.transpose();
    assert_eq!(transposed, mat);
}