
use matrix::Matrix;

#[test]
fn test_rref() {
    // Exemple :
    // [[1, 2, 3],
    //  [2, 4, 6],
    //  [1, 1, 1]]
    let mat = Matrix {
        data: [
            [1.0, 2.0, 3.0],
            [2.0, 4.0, 6.0],
            [1.0, 1.0, 1.0],
        ],
    };
    let rref = mat.rref();
    // Résultat attendu :
    // [[1, 0, -1],
    //  [0, 1,  2],
    //  [0, 0,  0]]
    let expected = Matrix {
        data: [
            [1.0, 0.0, -1.0],
            [0.0, 1.0,  2.0],
            [0.0, 0.0,  0.0],
        ],
    };
    assert_eq!(rref.data, expected.data);
}