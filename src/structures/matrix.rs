use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
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

// -----------------------------------------------------------------
// Add / Substract / Scale
// -----------------------------------------------------------------

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

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

impl<K, const M: usize, const N: usize> Add for Matrix<K, M, N>
where
    K: Add<Output = K> + Copy,
{
    type Output = Matrix<K, M, N>;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

impl<K, const M: usize, const N: usize> Sub for Matrix<K, M, N>
where
    K: Sub<Output = K> + Copy,
{
    type Output = Matrix<K, M, N>;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

impl<K, const M: usize, const N: usize> Mul for Matrix<K, M, N>
where
    K: Mul<Output = K> + Copy,
{
    type Output = Matrix<K, M, N>;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

impl<K, const M: usize, const N: usize> Div for Matrix<K, M, N>
where
    K: Div<Output = K> + Copy,
{
    type Output = Matrix<K, M, N>;

    fn div(self, other: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] / other.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

impl<K, const M: usize, const N: usize> PartialEq for Matrix<K, M, N>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                if self.data[i][j] != other.data[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

/// operator * beetween Matrix and number
impl<K, const M: usize, const N: usize> Mul<K> for Matrix<K, M, N>
where
    K: Mul<Output = K> + Copy,
{
    type Output = Matrix<K, M, N>;

    fn mul(self, scalar: K) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] * scalar;
            }
        }
        Matrix { data: result }
    }
}
