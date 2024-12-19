use crate::Vector;
use std::ops::{Mul, Sub};

pub fn cross_product<T>(u: Vector<T, 3>, v: Vector<T, 3>) -> Vector<T, 3>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy + std::fmt::Debug,
{
    let x = u.data[1] * v.data[2] - u.data[2] * v.data[1];
    let y = u.data[2] * v.data[0] - u.data[0] * v.data[2];
    let z = u.data[0] * v.data[1] - u.data[1] * v.data[0];
    Vector::new([x, y, z])
}
