use matrix::Matrix;

// -----------------------------------------------------------------
// Exercice 11 - Determinant
// -----------------------------------------------------------------

#[test]
fn test_determinant_2x2() {
    let mat = Matrix { data: [[2.0, 1.0],
                              [4.0, 3.0]] };
    assert_eq!(mat.determinant(), 2.0);
}

#[test]
fn test_determinant_3x3() {
    let mat = Matrix { data: [
        [1.0, 2.0, 3.0],
        [0.0, 1.0, 4.0],
        [5.0, 6.0, 0.0],
    ]};
    assert_eq!(mat.determinant(), 1.0);
}

#[test]
fn test_determinant_4x4() {
    let mat = Matrix { data: [
        [8.0, 5.0, -2.0, 4.0],
        [4.0, 2.5, 20.0,  4.0],
        [8.0, 5.0, 1.0, 4.0],
        [28.0, -4.0, 17.0, 1.0],
    ]};
    assert_eq!(mat.determinant(), 1032.0);
}
