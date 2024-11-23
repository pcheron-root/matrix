use matrix::Matrix;
use matrix::structures::vector::Vector;


#[test]
fn test_create_matrix(){
    let v1 = Vector::new([2, 3]);
    let v2 = Vector::new([1, 4]);
    let _ = Matrix::new([v1, v2]);
}

#[test]
fn test_print_vector() {
    let v1 = Vector::new([2, 3]);
    let v2 = Vector::new([1, 4]);
    let m = Matrix::new([v1, v2]);
    m.print();
}