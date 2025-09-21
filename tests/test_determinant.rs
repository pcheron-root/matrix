use matrix::Matrix;

// -----------------------------------------------------------------
// Exercice 11 - Determinant
// -----------------------------------------------------------------

#[test]
fn test_determinant_0() {
    let mat = Matrix { data: [[1.0, -1.0],
                              [-1.0, 1.0]] };
    assert_eq!(mat.determinant(), 0.0);
}

#[test]
fn test_determinant_1() {
    let mat = Matrix { data: [
        [2.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, 2.0],
    ]};
    assert_eq!(mat.determinant(), 8.0);
}

#[test]
fn test_determinant_2() {
    let mat = Matrix { data: [
        [8.0, 5.0, -2.0],
        [4.0, 7.0, 20.0],
        [7.0, 6.0, 1.0],
    ]};
    assert_eq!(mat.determinant(), -174.0);
}

#[test]
fn test_determinant_3() {
    let mat = Matrix { data: [
        [8.0, 5.0, -2.0, 4.0],
        [4.0, 2.5, 20.0,  4.0],
        [8.0, 5.0, 1.0, 4.0],
        [28.0, -4.0, 17.0, 1.0],
    ]};
    assert_eq!(mat.determinant(), 1032.0);
}

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

