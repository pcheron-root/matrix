use matrix::Vector;

#[test]
fn test_dot_prod_vector() {
    let v1 = Vector::new([0.0, 0.0]);
    let v2 = Vector::new([1.0, 1.0]);
    assert_eq!(v1.dot(&v2), 0.0);

    let v3 = Vector::new([1.0, 1.0]);
    let v4 = Vector::new([1.0, 1.0]);
    assert_eq!(v3.dot(&v4), 2.0);

    let v5 = Vector::new([-1.0, 6.0]);
    let v6 = Vector::new([3.0, 2.0]);
    assert_eq!(v5.dot(&v6), 9.0);
}
