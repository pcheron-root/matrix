use std::ops::Mul;
use std::ops::Sub;

pub fn lerp<
    T: std::ops::Sub
        + Clone
        + std::ops::Add<<<T as std::ops::Sub>::Output as std::ops::Mul<K>>::Output, Output = T>,
    K,
>(
    x: T,
    y: T,
    c: K,
) -> T
where
    <T as Sub>::Output: Mul<K>,
{
    x.clone() + (y - x) * c
}
