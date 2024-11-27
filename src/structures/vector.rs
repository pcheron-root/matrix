use std::ops::MulAssign;

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
    /// add will change the Vector
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

// fn linear_combination<
//     T: std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + Copy,
//     const N: usize,
// >(
//     u: &[Vector<T, N>],
//     coefs: &[T],
// ) -> Vector<T, N> {
//     for i in 0..N {
//         u[i].scl(coefs[i]);
//     }
//     u[0]
// }
