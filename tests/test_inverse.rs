
// -----------------------------------------------------------------
// Exercice 12 - Inverse matrix
// -----------------------------------------------------------------

use matrix::Matrix;

// #[test]
// fn test_inverse_2x2() {
//     let mat = Matrix {
//         data: [[2.0, 1.0],
//                [4.0, 3.0]],
//     };

//     let inv = mat.inverse().expect("This matrix can't be reversed");

//     // Produit A * A^{-1}
//     let prod = mat * inv;

//     // Vérifie que le résultat est proche de l'identité
//     let id = Matrix::<f64, 2, 2>::identity();
//     for i in 0..2 {
//         for j in 0..2 {
//             eprintln!("data {} {} : {} et {}", i, j, prod.data[i][j], id.data[i][j]);
//             assert!((prod.data[i][j] - id.data[i][j]).abs() < 1e-10);
//         }
//     }
// }

// #[test]
// fn test_inverse_3x3() {
//     let mat = Matrix {
//         data: [
//             [1.0, 2.0, 3.0],
//             [0.0, 1.0, 4.0],
//             [5.0, 6.0, 0.0],
//         ],
//     };

//     let inv = mat.inverse().expect("Matrice inversible attendue");
//     let prod = mat * inv;

//     let id = Matrix::<f64, 3, 3>::identity();
//     for i in 0..3 {
//         for j in 0..3 {
//             assert!((prod.data[i][j] - id.data[i][j]).abs() < 1e-10);
//         }
//     }
// }

#[test]
fn test_inverse_non_invertible() {
    // Matrice singulière : det = 0
    let mat = Matrix {
        data: [[1.0, 2.0],
               [2.0, 4.0]],
    };

    assert!(mat.inverse().is_none());
}
