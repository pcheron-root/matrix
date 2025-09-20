
use matrix::Matrix;

fn main() {
    println!("Hello, world!");

    // let mat = Matrix::new([[2.0, 1.0],
    //     [4.0, 3.0]]);
    // let ref_mat = mat.row_echelon();
    // assert_eq!(ref_mat, Matrix::new([[4.0, 3.0],
    //     [0.0, -0.5]]));

    let mat = Matrix::new([[1.0, 2.0, 1.0],
        [2.0, 4.0, -1.0],
        [-1.0, -2.0, 5.0]]);
    let ref_mat = mat.row_echelon();
    // Une forme échelonnée possible
    assert_eq!(ref_mat, Matrix::new([[1.0, 2.0, 1.0],
        [0.0, 0.0, -3.0],
        [0.0, 0.0, 0.0]]));

}
