use matrix::Vector;

fn epsilon(x: f32, y:f32) -> bool{
    if x - y < 0.0001 && x - y > -0.0001 {
        return true;
    }
    false
}

#[test]
fn  test_cosine_1() {
    let u = Vector::new([1.0, 0.0]);
    let v = Vector::new([1.0, 0.0]);
    let cosine = u.angle_cos(&v);
    assert_eq!(cosine, 1.0);
}

#[test]
fn  test_cosine_2() {
    let u = Vector::new([1.0, 0.0]);
    let v = Vector::new([0.0, 1.0]);
    let cosine = u.angle_cos(&v);
    assert_eq!(cosine, 0.0);
}

#[test]
fn  test_cosine_3() {
    let u = Vector::new([-1.0, 1.0]);
    let v = Vector::new([1.0, -1.0]);
    let cosine = u.angle_cos(&v);
    assert_eq!(epsilon(cosine, -1.0), true);
}

#[test]
fn  test_cosine_4() {
    let u = Vector::new([2.0, 1.0]);
    let v = Vector::new([4.0, 2.0]);
    let cosine = u.angle_cos(&v);
    assert_eq!(epsilon(cosine, 1.0), true);
}

#[test]
fn  test_cosine_5() {
    let u = Vector::new([1.0, 2.0, 3.0]);
    let v = Vector::new([4.0, 5.0, 6.0]);
    let cosine = u.angle_cos(&v);
    assert_eq!(epsilon(cosine, 0.974631846), true);
}
