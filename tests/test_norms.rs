use matrix::Vector;

// -----------------------------------------------------------------
// Exercice 04 - Norm
// -----------------------------------------------------------------

#[test]
fn test_manha_norm() {
    let v1 = Vector::new([0.0, 0.0, 0.0]);
    assert_eq!(v1.manha_norm(), 0.0);
    let v2 = Vector::new([1.0, 2.0, 3.0]);
    assert_eq!(v2.manha_norm(), 6.0);
    let v3 = Vector::new([-1.0, -2.0]);
    assert_eq!(v3.manha_norm(), 3.0);

    let v1 = Vector::new([0., 0., 0.]);
    assert_eq!(v1.manha_norm(), 0.);
    let v2 = Vector::new([1., 2., 3.]);
    assert_eq!(v2.manha_norm(), 6.);
    let v3 = Vector::new([-1., -2.]);
    assert_eq!(v3.manha_norm(), 3.);
}

#[test]
fn test_eucli_norm() {
    let v1 = Vector::new([0.0, 0.0, 0.0]);
    assert_eq!(v1.eucli_norm(), 0.0);
    let v2 = Vector::new([1.0, 2.0, 3.0]);
    assert_eq!(v2.eucli_norm(), 3.7416573867739413);
    let v3 = Vector::new([-1.0, -2.0]);
    assert_eq!(v3.eucli_norm(), 2.23606797749979);
}

#[test]
fn test_suprem_norm() {
    let v1 = Vector::new([0.0, 0.0, 0.0]);
    assert_eq!(v1.suprem_norm(), 0.0);
    let v2 = Vector::new([1.0, 2.0, 3.0]);
    assert_eq!(v2.suprem_norm(), 3.);
    let v3 = Vector::new([-1.0, -2.0]);
    assert_eq!(v3.suprem_norm(), 2.);
}
