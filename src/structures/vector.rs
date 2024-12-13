use num_traits::{One, Zero};
use std::fmt;
use std::ops::MulAssign;
use std::ops::{Add, Div, Mul, Sub};

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

// -----------------------------------------------------------------
// ADD / SUB / SCALE
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
// linear combination
// -----------------------------------------------------------------

impl<T: Clone, const N: usize> Clone for Vector<T, N> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

pub fn linear_combination<T: Clone + Copy + MulAssign + std::ops::AddAssign, const N: usize>(
    u: &[Vector<T, N>],
    c: &[T],
) -> Vector<T, N> {
    let mut ret = u[0].clone();
    ret.scl(c[0]);
    for i in 1..N {
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
// Dot product
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
// 1-norm / 2-norm / inf-norm
// -----------------------------------------------------------------

impl<T, const N: usize> Vector<T, N>
where
    T: Zero + Copy + PartialOrd + std::ops::Sub<Output = T>,
{
    pub fn norm_1(&self) -> T {
        let mut res = T::zero();
        for i in 0..N {
            if self.data[i] < T::zero() {
                res = res - self.data[i];
            } else {
                res = res + self.data[i];
            }
        }
        res
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Into<f32> + Copy,
{
    pub fn norm(&self) -> f32 {
        let mut res: f32 = 0.0;
        for i in 0..N {
            let x: f32 = self.data[i].into();
            res = res + x * x;
        }
        res.sqrt()
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
