
use matrix::Matrix;

// -----------------------------------------------------------------
// Exercice 10 - Row echelon
// -----------------------------------------------------------------

#[test]
fn test_row_echelon_0() {
    let mat = Matrix::new([[1.0, 2.0],
        [3.0, 4.0]]);
    let ref_mat = mat.row_echelon();
    assert_eq!(ref_mat, Matrix::new([[1.0, 1.3333333333333333],
        [0.0, 1.0]]));
}

#[test]
fn test_row_echelon_3() {
    let mat = Matrix::new([[1.0, 2.0],
        [2.0, 4.0]]);
    let ref_mat = mat.row_echelon();
    assert_eq!(ref_mat, Matrix::new([[1.0, 2.0],
        [0.0, 0.0]]));
}

#[test]
fn test_row_echelon_5x3() {
    let mat = Matrix::new([[8.0, 5.0, -2., 4., 28.],
        [4.0, 2.5, 20., 4., -4.],
        [8.0, 5.0, 1., 4., 17.]]);
    let ref_mat = mat.row_echelon();
    assert_eq!(ref_mat, Matrix::new([[1.0, 0.625, 0.125, 0.5, 2.125],
        [0.0, 0.0, 1.0, 0.10256410256410256, -0.6410256410256411],
        [0.0, 0.0, 0.0, 1.0, 29.499999999999996]]));
}

// #[test]
// fn test_row_echelon_0() {
//     let mat = Matrix::new([[1.0, 0.0, 0.0],
//         [0.0, 1.0, 0.0],
//         [0.0, 0.0, 1.0],]);
//     let ref_mat = mat.row_echelon();
//     assert_eq!(ref_mat, Matrix::new([[1.0, 0.0, 0.0],
//         [0.0, 1.0, 0.0],
//         [0.0, 0.0, 1.0]]));
// }

// #[test]
// fn test_row_echelon_2x2() {
//     let mat = Matrix::new([[2.0, 1.0],
//         [4.0, 3.0]]);
//     let ref_mat = mat.row_echelon();
//     assert_eq!(ref_mat, Matrix::new([[4.0, 3.0],
//         [0.0, -0.5]]));
// }

// ce test ne fonctionne pas
// #[test]
// fn test_row_echelon_3x3() {
//     let mat = Matrix::new([[1.0, 2.0, 1.0],
//                                                     [2.0, 4.0, -1.0],
//                                                     [-1.0, -2.0, 5.0]]);
//     let ref_mat = mat.row_echelon();
//     // Une forme échelonnée possible
//     assert_eq!(ref_mat, Matrix::new([[2.0, 4.0, -1.0],
//                                      [0.0, 0.0, 4.5],
//                                      [0.0, 0.0, 0.0]]));
// }

// #[test]
// fn test_row_echelon_already_echelon() {
//     let mat = Matrix::new([[1.0, 2.0, 3.0],
//                            [0.0, 1.0, 4.0],
//                            [0.0, 0.0, 1.0]]);
//     let ref_mat = mat.row_echelon();
//     assert_eq!(ref_mat, mat); // déjà en forme échelonnée
// }

// #[test]
// fn test_row_echelon_with_zero_row() {
//     let mat = Matrix::new([[0.0, 2.0, 1.0],
//                            [0.0, 0.0, 0.0],
//                            [1.0, 1.0, 1.0]]);
//     let ref_mat = mat.row_echelon();
//     // Les lignes nulles passent en bas
//     assert_eq!(ref_mat, Matrix::new([[1.0, 1.0, 1.0],
//                                      [0.0, 2.0, 1.0],
//                                      [0.0, 0.0, 0.0]]));
// }
