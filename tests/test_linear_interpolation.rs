use matrix::linear_interpolation::lerp;
// use matrix::Matrix;
// use matrix::Vector;

#[test]
fn test_linear_interpolation_float() {
    let x = 2.0;
    let y = 8.0;
    assert_eq!(lerp(x, y, 0.5), 5.0);
    assert_eq!(lerp(x, y, 0.2), 3.2);
}

#[test]
fn test_linear_interpolation_int() {
    let x = 2;
    let y = 8;
    assert_eq!(lerp(x, y, 2), 14);
    assert_eq!(lerp(x, y, 3), 20);
    assert_eq!(lerp(x, y, -1), -4);
}

// #[test]
// fn test_linear_interpolation_vector() {
//     let v1 = Vector::new([2.0, 2.0]);
//     let v2 = Vector::new([4.0, 4.0]);
//     let v3 = Vector::new([3.0, 3.0]);
//     assert_eq!(lerp(v1, v2, 0.5), v3);
//     // assert_eq!(lerp(v1, v2, 3), 20);
//     // assert_eq!(lerp(v1, v2, -1), -4);
// }
