use matrix::Vector;

#[test]
fn test_create_vector() {
    let _ = Vector::new([42]);
}

#[test]
fn test_print_vector() {
    let v = Vector::new([42]);
    v.print();
}