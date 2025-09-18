use matrix::Matrix;

// -----------------------------------------------------------------
// Exercice 08 - Trace
// -----------------------------------------------------------------

#[test]
fn test_trace_2x2() {
    let mat = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    assert_eq!(mat.trace(), 1.0 + 4.0);
}

#[test]
fn test_trace_3x3() {
    let mat = Matrix::new([[1.0, 2.0, 3.0],
                           [4.0, 5.0, 6.0],
                           [7.0, 8.0, 9.0]]);
    assert_eq!(mat.trace(), 1.0 + 5.0 + 9.0);
}

#[test]
fn test_trace_identity_4x4() {
    let mat = Matrix::new([[1.0, 0.0, 0.0, 0.0],
                           [0.0, 1.0, 0.0, 0.0],
                           [0.0, 0.0, 1.0, 0.0],
                           [0.0, 0.0, 0.0, 1.0]]);
    assert_eq!(mat.trace(), 4.0);
}

#[test]
fn test_trace_zero_3x3() {
    let mat = Matrix::new([[0.0, 0.0, 0.0],
                           [0.0, 0.0, 0.0],
                           [0.0, 0.0, 0.0]]);
    assert_eq!(mat.trace(), 0.0);
}

#[test]
fn test_trace_negative_elements() {
    let mat = Matrix::new([[-1.0, 2.0],
                           [3.0, -4.0]]);
    assert_eq!(mat.trace(), -1.0 + (-4.0));
}

