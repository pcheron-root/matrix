use matrix::cross_product::cross_product;
use matrix::Vector;

#[test]
fn test_cross_product_1() {
    let u = Vector::new([0.0, 0.0, 1.0]);
    let v = Vector::new([1.0, 0.0, 0.0]);
    let w = Vector::new([0.0, 1.0, 0.0]);
    assert_eq!(cross_product(u, v), w);
}

#[test]
fn test_cross_product_2() {
    let u = Vector::new([1.0, 2.0, 3.0]);
    let v = Vector::new([4.0, 5.0, 6.0]);
    let w = Vector::new([-3.0, 6.0, -3.0]);
    assert_eq!(cross_product(u, v), w);
}

#[test]
fn test_cross_product_3() {
    let u = Vector::new([4.0, 2.0, -3.0]);
    let v = Vector::new([-2.0, -5.0, 16.0]);
    let w = Vector::new([17.0, -58.0, -16.0]);
    assert_eq!(cross_product(u, v), w);
}
