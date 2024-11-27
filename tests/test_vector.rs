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

// add / substract / scale test features

#[test]
fn test_add_vector() {
    let mut v1 = Vector::new([13, 4, 2]);
    let v2 = Vector::new([29, 1, 1]);
    v1.add(&v2);
    assert_eq!(v1.data[0], 42);
    assert_eq!(v1.data[1], 5);
    assert_eq!(v1.data[2], 3);

    let mut v3 = Vector::new([13.0, 4.0, 2.0]);
    let v4 = Vector::new([29.0, 1.0, 1.0]);
    v3.add(&v4);
    assert_eq!(v3.data[0], 42.0);
    assert_eq!(v3.data[1], 5.0);
    assert_eq!(v3.data[2], 3.0);
}

#[test]
fn test_sub_vector() {
    let mut v1 = Vector::new([13, 4, 2]);
    let v2 = Vector::new([12, 1, 1]);
    v1.sub(&v2);
    assert_eq!(v1.data[0], 1);
    assert_eq!(v1.data[1], 3);
    assert_eq!(v1.data[2], 1);

    let mut v3 = Vector::new([4.0, 64.2, 2.4]);
    let v4 = Vector::new([12.0, 1.4, 1.26]);
    v3.sub(&v4);
    assert_eq!(v3.data[0], -8.0);
    assert_eq!(v3.data[1], 62.800000000000004);
    assert_eq!(v3.data[2], 1.14);
}

#[test]
fn test_scl_vector() {
    let mut v1 = Vector::new([13, 4, 2]);
    let x = 2;
    v1.scl(x);
    assert_eq!(v1.data[0], 26);
    assert_eq!(v1.data[1], 8);
    assert_eq!(v1.data[2], 4);

    let mut v2 = Vector::new([5.2, 0.5, 1.12]);
    let x = 2.0;
    v2.scl(x);
    assert_eq!(v2.data[0], 10.4);
    assert_eq!(v2.data[1], 1.0);
    assert_eq!(v2.data[2], 2.24);
}
