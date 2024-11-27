// use super::vector::Vector;
use std::fmt::Debug;
// use std::ops::AddAssign;

pub struct Matrix<K, const M: usize, const N: usize> {
    pub data: [[K; N]; M],
}

impl<K, const M: usize, const N: usize> Matrix<K, M, N> {}

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: Default + Copy,
{
    pub fn new(data: [[K; N]; M]) -> Self {
        Self { data }
    }

    pub fn shape(&self) -> [usize; 2] {
        [*(&self.data.len()), *(&self.data[0].len())]
    }

    pub fn is_square(&self) -> bool {
        let s = &self.shape();
        if s[0] == s[1] {
            return true;
        }
        false
    }

    // to do : a method that convert Matrix in Vector
    // pub fn to_vector(self) -> Vector<T, N> {
    //     let mut components = Vec::with_capacity(M * N);
    //     for row in self.data.iter() {
    //         components.extend_from_slice(row);
    //     }
    // }
}

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: Debug,
{
    pub fn print(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }
}

// add / substract / scale

impl<
        K: std::fmt::Debug + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign,
        const M: usize,
        const N: usize,
    > Matrix<K, M, N>
where
    K: Copy,
{
    pub fn add(&mut self, m: &Matrix<K, M, N>) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] += m.data[i][j];
            }
        }
    }
    pub fn sub(&mut self, m: &Matrix<K, M, N>) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] -= m.data[i][j];
            }
        }
    }
    pub fn scl(&mut self, x: K) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] *= x;
            }
        }
    }
}
