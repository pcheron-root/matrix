
// -----------------------------------------------------------------
// Exercice 12 - Inverse matrix
// -----------------------------------------------------------------

use matrix::Matrix;

#[test]
fn test_inverse_2x2() {
    let mat = Matrix {
        data: [[2.0, 1.0],
               [4.0, 3.0]],
    };

    let inv = mat.inverse().expect("This matrix can't be reversed");

    let sol = Matrix {
        data: [[1.5, -0.5],
               [-2.0, 1.0]],
    };
    assert_eq!(inv, sol);
}

#[test]
fn test_inverse_3x3() {
    let mat = Matrix {
        data: [
            [1.0, 2.0, 3.0],
            [0.0, 1.0, 4.0],
            [5.0, 6.0, 0.0],
        ],
    };
    let inv = mat.inverse().expect("This matrix can't be reversed");


    let sol = Matrix {
        data: [[-24., 18., 5.],
               [20.0, -15.0, -4.],
               [-5.0, 4.0, 1.]],
    };
    assert_eq!(inv, sol);
}

#[test]
fn test_inverse_non_invertible() {
    // derterminant est nul (1 * 4 - 2 * 2) = 0
    let mat = Matrix {
        data: [[1.0, 2.0],
               [2.0, 4.0]],
    };

    assert!(mat.inverse().is_none());
}
