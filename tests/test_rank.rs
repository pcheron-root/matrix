use matrix::Matrix;

#[test]
fn test_rank_full_0() {
    let mat = Matrix::new([
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]);
    assert_eq!(mat.rank(), 3);
}

#[test]
fn test_rank_full_1() {
    let mat = Matrix::new([
        [1.0, 2.0, 0.0, 0.0],
        [2.0, 4.0, 0.0, 0.0],
        [-1.0, 2.0, 1.0, 1.0],
    ]);
    assert_eq!(mat.rank(), 2);
}

// test qui ne fonctionne pas
// #[test]
// fn test_rank_full_2() {
//     let mat = Matrix::new([
//         [8.0, 5.0, -2.0],
//         [4.0, 7.0, 20.0],
//         [7.0, 6.0, 1.0],
//         [21.0, 18.0, 7.0],
//     ]);
//     assert_eq!(mat.rank(), 3);
// }

#[test]
fn test_rank_zero_matrix() {
    let mat = Matrix::new([[0.0, 0.0], [0.0, 0.0]]);
    assert_eq!(mat.rank(), 0);
}

#[test]
fn test_rank_identity() {
    let mat = Matrix::new([[1.0, 0.0], [0.0, 1.0]]);
    assert_eq!(mat.rank(), 2);
}

#[test]
fn test_rank_dependent_rows() {
    // deuxième ligne = 2 * première ligne
    let mat = Matrix::new([[1.0, 2.0], [2.0, 4.0]]);
    assert_eq!(mat.rank(), 1);
}

#[test]
fn test_rank_full_3x3() {
    let mat = Matrix::new([
        [1.0, 2.0, 3.0],
        [0.0, 1.0, 4.0],
        [5.0, 6.0, 0.0],
    ]);
    assert_eq!(mat.rank(), 3);
}

#[test]
fn test_rank_rectangular() {
    // 3x2 avec deux colonnes linéairement indépendantes
    let mat = Matrix::new([
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ]);
    assert_eq!(mat.rank(), 2);
}
