use num_traits::Zero;
use std::fmt::{self, Display};
use std::ops::MulAssign;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::Float;
use num_traits::Signed;

pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T: std::fmt::Debug, const N: usize> Vector<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }

    pub fn size(&self) -> usize {
        N
    }

    pub fn print(&self) {
        println!("{:?}", self.data);
    }
}

impl<T: fmt::Display, const N: usize> fmt::Display for Vector<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, val) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }
        write!(f, "]")
    }
}

// -----------------------------------------------------------------
// Exercice 00 - implementing add / sub / scale
// -----------------------------------------------------------------

impl<T: std::ops::AddAssign, const N: usize> Vector<T, N>
where
    T: Copy,
{
    /// doc
    pub fn add(&mut self, v: &Vector<T, N>) {
        for i in 0..N {
            self.data[i] += v.data[i];
        }
    }
}

impl<T: std::ops::SubAssign, const N: usize> Vector<T, N>
where
    T: Copy,
{
    pub fn sub(&mut self, v: &Vector<T, N>) {
        for i in 0..N {
            self.data[i] -= v.data[i];
        }
    }
}

impl<T: std::ops::MulAssign, const N: usize> Vector<T, N>
where
    T: Copy,
{
    pub fn scl(&mut self, x: T) {
        for i in 0..N {
            self.data[i] *= x;
        }
    }
}

// -----------------------------------------------------------------
// Exercice 01 - implementing linear combination
// -----------------------------------------------------------------

impl<T: Clone, const N: usize> Clone for Vector<T, N> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

// O(n)
pub fn linear_combination<T: Clone + Copy + MulAssign + std::ops::AddAssign + Display, const N: usize>(
    u: &[Vector<T, N>],
    c: &[T],
) -> Vector<T, N> {

    assert_eq!(
        u.len(),
        c.len(),
        "linear_combination: number of vectors ({}) does not match number of coefficients ({})",
        u.len(),
        c.len()
    );

    let mut ret = u[0].clone();
    ret.scl(c[0]);
    for i in 1..u.len() {
        let mut elem = u[i].clone();
        elem.scl(c[i]);
        ret.add(&elem);
    }
    ret
}

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn add(self, other: Self) -> Self::Output {
        let mut result = [self.data[0]; N];
        for i in 0..N {
            result[i] = self.data[i] + other.data[i];
        }
        Vector { data: result }
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = [self.data[0]; N];
        for i in 0..N {
            result[i] = self.data[i] - other.data[i];
        }
        Vector { data: result }
    }
}

impl<T, const N: usize> Mul for Vector<T, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = [self.data[0]; N];
        for i in 0..N {
            result[i] = self.data[i] * other.data[i];
        }
        Vector { data: result }
    }
}

impl<T, const N: usize> Div for Vector<T, N>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn div(self, other: Self) -> Self::Output {
        let mut result = [self.data[0]; N];
        for i in 0..N {
            result[i] = self.data[i] / other.data[i];
        }
        Vector { data: result }
    }
}

impl<T, const N: usize> PartialEq for Vector<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

/// operator * beetween Vector and number
impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn mul(self, scalar: T) -> Self::Output {
        let mut result = self.data;
        for i in 0..N {
            result[i] = self.data[i] * scalar;
        }
        Vector { data: result }
    }
}

// -----------------------------------------------------------------
// Debug trait
// -----------------------------------------------------------------

impl<T, const N: usize> fmt::Debug for Vector<T, N>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Vector").field("data", &self.data).finish()
    }
}

// -----------------------------------------------------------------
// Exercice 03 - Implementing Dot product (produit scalaire)
// -----------------------------------------------------------------

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    pub fn dot(&self, v: &Self) -> T {
        let mut result = self.data[0] * v.data[0];
        for i in 1..N {
            result = result + (self.data[i] * v.data[i]);
        }
        result
    }
}

// -----------------------------------------------------------------
// Exercice 04 - Norms
// -----------------------------------------------------------------

impl<T, const N: usize> Vector<T, N>
where
    T: Zero + Copy + PartialOrd + Float + std::ops::Sub<Output = T>,
{
    pub fn manha_norm(&self) -> T {
        let mut res = T::zero();
        for i in 0..N {
            if self.data[i] < T::zero() {
                res = res - self.data[i];
            } else {
                res = res + self.data[i].abs();
            }
        }
        res
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Float + Copy,
{
    pub fn eucli_norm(&self) -> T {
        let mut res = T::zero();
        for i in 0..N {
            res = res + self.data[i] * self.data[i];
        }
        res.sqrt()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Signed + Copy + PartialOrd,
{
    pub fn suprem_norm(&self) -> T {
        let mut res = T::zero();
        for i in 0..N {
            let val = self.data[i].abs();
            if val > res {
                res = val;
            }
        }
        res
    }
}

// impl<T, const N: usize> Vector<T, N>
// where
//     T: Zero + One + std::ops::Neg<Output = T> + std::cmp::PartialOrd,
// {
//     pub fn norm_inf(&self) -> T {
//         let mut max;
//         if self.data[0] < T::zero() {
//             max = -self.data[0];
//         } else {
//             max = self.data[0];
//         }

//         for i in 1..N {
//             if self.data[i] < T::zero() && (-self.data[i] > max) {
//                 max = -self.data[i];
//             } else if max < self.data[i] {
//             }
//         }
//         max
//     }
// }


// -----------------------------------------------------------------
// Exercice 05 - Implementing cosine
// -----------------------------------------------------------------
// cosine of the angle between two vectors
// dot product / norm2 u * norm2 v
// -----------------------------------------------------------------

// const usize a la place de N ?
impl<T, const N: usize> Vector<T, N>
where
    T: Float + Into<f32> + Copy + Add<Output = T> + Mul<Output = T> + std::ops::Div<f32, Output = f32>,
{
    pub fn angle_cos(&self, v: &Self) -> T {
        let dot_prod = self.dot(v);
        let norm_self = self.eucli_norm();
        let norm_other_v = v.eucli_norm();
        if norm_self == T::zero() || norm_other_v == T::zero() {
            panic!("Cannot compute angle with zero-length vector");
        }
        dot_prod / (self.eucli_norm() * v.eucli_norm())
    }
}
