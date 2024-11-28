use std::ops::{Add, Mul, Sub};

pub fn lerp<T, K>(x: T, y: T, c: K) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<K, Output = T> + Clone,
{
    x.clone() + (y - x) * c
}
