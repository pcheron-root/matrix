use crate::Vector;
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

// -----------------------------------------------------------------
// Exercice 07 - Mult by Matrix and Vector
// -----------------------------------------------------------------

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: Mul<K, Output = K> + Add<Output = K> + Copy + Default,
{
    pub fn mul_vec(&self, vector: &Vector<K, N>) -> Vector<K, M> {
        let mut result = [K::default(); M];

        for i in 0..M {
            let mut sum = K::default();
            for j in 0..N {
                sum = sum + self.data[i][j] * vector.data[j];
            }
            result[i] = sum;
        }

        Vector { data: result }
    }

    pub fn mul_mat<const P: usize>(&self, other: &Matrix<K, N, P>) -> Matrix<K, M, P> {
        let mut result = [[K::default(); P]; M];

        for i in 0..M {
            for j in 0..P {
                let mut sum = K::default();
                for k in 0..N {
                    sum = sum + self.data[i][k] * other.data[k][j];
                }
                result[i][j] = sum;
            }
        }

        Matrix { data: result }
    }
}

// -----------------------------------------------------------------
// Exercice 08 - Implementing Trace
// -----------------------------------------------------------------

impl<K, const N: usize> Matrix<K, N, N>
where
    K: Copy + Default + std::ops::Add<Output = K>,
{
    pub fn trace(&self) -> K {
        let mut sum = K::default();
        for i in 0..N {
            sum = sum + self.data[i][i];
        }
        sum
    }
}

// -----------------------------------------------------------------
// Exercice 09 - Implement transpose matrix
// -----------------------------------------------------------------

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: Copy,
{
    pub fn transpose(&self) -> Matrix<K, N, M> {
        let mut result = [[self.data[0][0]; M]; N];
        for i in 0..M {
            for j in 0..N {
                result[j][i] = self.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

// -----------------------------------------------------------------
// Exercice 10 - Implement Row echelon form (forme echelonee)
// -----------------------------------------------------------------

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: Copy
        + Default
        + PartialEq
        + std::ops::Div<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Mul<Output = K>
        + std::fmt::Debug,
{
    pub fn row_echelon(&self) -> Matrix<K, M, N> {
        let mut mat = self.data; // copie pour ne pas modifier self
        let mut lead = 0;

        for r in 0..M {
            if lead >= N {
                break;
            }

            // Trouver la ligne avec un pivot non nul
            let mut i = r;
            while i < M && mat[i][lead] == K::default() {
                i += 1;
            }

            if i == M {
                lead += 1;
                continue;
            }

            // Échanger lignes r et i
            if i != r {
                mat.swap(r, i);
            }

            // Mettre les éléments sous le pivot à zéro
            for j in (r + 1)..M {
                if mat[j][lead] != K::default() {
                    let factor = mat[j][lead] / mat[r][lead];
                    for k in lead..N {
                        mat[j][k] = mat[j][k] - factor * mat[r][k];
                    }
                }
            }

            lead += 1;
        }

        Matrix { data: mat }
    }
}

