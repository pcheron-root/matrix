use matrix::Vector;

#[test]
fn test_norm_1() {
    let v1 = Vector::new([0.0, 0.0, 0.0]);
    assert_eq!(v1.norm_1(), 0.0);
    let v2 = Vector::new([1.0, 2.0, 3.0]);
    assert_eq!(v2.norm_1(), 6.0);
    let v3 = Vector::new([-1.0, -2.0]);
    assert_eq!(v3.norm_1(), 3.0);

    let v1 = Vector::new([0, 0, 0]);
    assert_eq!(v1.norm_1(), 0);
    let v2 = Vector::new([1, 2, 3]);
    assert_eq!(v2.norm_1(), 6);
    let v3 = Vector::new([-1, -2]);
    assert_eq!(v3.norm_1(), 3);
}

#[test]
fn test_norm_2() {
    let v1 = Vector::new([0.0, 0.0, 0.0]);
    assert_eq!(v1.norm(), 0.0);
    let v2 = Vector::new([1.0, 2.0, 3.0]);
    assert_eq!(v2.norm(), 3.74165738);
    let v3 = Vector::new([-1.0, -2.0]);
    assert_eq!(v3.norm(), 2.236067977);
}
