use crate::Vector;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use num_traits::Float;

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
// Exercice 00 - Implementing Add / Substract / Scale
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
        + Sized
        + Float
        + std::fmt::Debug,
{
    pub fn row_echelon(&self) -> Matrix<K, M, N> {
        let mut mat = self.data; // copie pour ne pas modifier self
        let mut lead = 0;

        for r in 0..M {
            if lead >= N {
                break;
            }

            // Recherche du meilleur pivot (en supposant que K supporte abs() et PartialOrd)
            let mut best_pivot = None;
            let mut best_abs = K::default(); // ou une valeur minimale appropriée

            for i in r..M {
                if mat[i][lead] != K::default() {
                    let current_abs = mat[i][lead].abs(); // nécessite que K implémente abs()
                    if best_pivot.is_none() || current_abs > best_abs {
                        best_abs = current_abs;
                        best_pivot = Some(i);
                    }
                }
            }

            let pivot_row = match best_pivot {
                Some(i) => i,
                None => {
                    lead += 1;
                    continue; // Pas de pivot trouvé dans cette colonne
                }
            };

            // Échanger lignes r et pivot_row
            if pivot_row != r {
                mat.swap(r, pivot_row);
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

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: Copy
        + Default
        + PartialEq
        + std::ops::Div<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Mul<Output = K>
        + std::ops::Add<Output = K>
        + std::ops::Neg<Output = K>
        + std::fmt::Debug,
{
    pub fn rref(&self) -> Matrix<K, M, N> {
        let mut mat = self.data;
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
                if lead >= N {
                    break;
                }
                continue;
            }

            // Échanger lignes r et i
            if i != r {
                mat.swap(r, i);
            }

            // Normaliser la ligne pivot
            let pivot = mat[r][lead];
            if pivot != K::default() {
                for k in lead..N {
                    mat[r][k] = mat[r][k] / pivot;
                }
            }

            // Mettre à zéro toutes les autres lignes (au-dessus et au-dessous)
            for j in 0..M {
                if j != r && mat[j][lead] != K::default() {
                    let factor = mat[j][lead];
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

// -----------------------------------------------------------------
// Exercice 11 - Implementing determinant
// -----------------------------------------------------------------

impl<K> Matrix<K, 2, 2>
where
    K: Copy
        + std::ops::Mul<Output = K>
        + std::ops::Sub<Output = K>,
{
    pub fn determinant(&self) -> K {
        self.data[0][0] * self.data[1][1]
            - self.data[0][1] * self.data[1][0]
    }
}

impl<K> Matrix<K, 3, 3>
where
    K: Copy
        + std::ops::Mul<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Add<Output = K>,
{
    pub fn determinant(&self) -> K {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1])
            - self.data[0][1] * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0])
            + self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }
}

impl<K> Matrix<K, 4, 4>
where
    K: Copy
        + std::ops::Mul<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Add<Output = K>
        + std::ops::Neg<Output = K>
        + Default,
{
    pub fn determinant(&self) -> K {
        let mut det = K::default();
        for j in 0..4 {
            // Construire la sous-matrice 3x3
            let mut sub = [[K::default(); 3]; 3];
            for row in 1..4 {
                let mut col_sub = 0;
                for col in 0..4 {
                    if col != j {
                        sub[row - 1][col_sub] = self.data[row][col];
                        col_sub += 1;
                    }
                }
            }
            let submat = Matrix { data: sub };
            let cofactor = if j % 2 == 0 {
                self.data[0][j] * submat.determinant()
            } else {
                -(self.data[0][j] * submat.determinant())
            };
            det = det + cofactor;
        }
        det
    }
}

// -----------------------------------------------------------------
// Exercice 12 - Implementing inverse matrix calculus
// -----------------------------------------------------------------

impl<const N: usize> Matrix<f64, N, N> {
    /// Retourne l'identité NxN
    pub fn identity() -> Self {
        let mut data = [[0.0; N]; N];
        for i in 0..N {
            data[i][i] = 1.0;
        }
        Matrix { data }
    }

    /// Calcule l'inverse via Gauss-Jordan
    pub fn inverse(&self) -> Option<Matrix<f64, N, N>> {
        // Construire la matrice augmentée [A | I]
        let mut aug = vec![vec![0.0; 2 * N]; N];
        for i in 0..N {
            for j in 0..N {
                aug[i][j] = self.data[i][j];
            }
            aug[i][N + i] = 1.0;
        }

        // Gauss-Jordan
        for i in 0..N {
            // Pivot : chercher une ligne avec un coefficient non nul
            if aug[i][i] == 0.0 {
                let mut found = false;
                for k in i + 1..N {
                    if aug[k][i] != 0.0 {
                        aug.swap(i, k);
                        found = true;
                        break;
                    }
                }
                if !found {
                    return None; // Matrice non inversible
                }
            }

            // Normaliser la ligne pivot
            let pivot = aug[i][i];
            for j in 0..2 * N {
                aug[i][j] /= pivot;
            }

            // Éliminer les autres lignes
            for k in 0..N {
                if k != i {
                    let factor = aug[k][i];
                    for j in 0..2 * N {
                        aug[k][j] -= factor * aug[i][j];
                    }
                }
            }
        }

        // Extraire la partie droite comme inverse
        let mut inv = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                inv[i][j] = aug[i][N + j];
            }
        }

        Some(Matrix { data: inv })
    }
}

// -----------------------------------------------------------------
// Exercice 13 - Implementing rank
// -----------------------------------------------------------------

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: PartialEq
    + From<u8> 
    + Copy
    + Default
    + PartialEq
    + std::ops::Div<Output = K>
    + std::ops::Sub<Output = K>
    + std::ops::Mul<Output = K>
    + std::fmt::Debug
    + Float,
{
    pub fn rank(&self) -> usize {
        let echelon = self.row_echelon();
        let zero = K::from(0u8);

        echelon
            .data
            .iter()
            .filter(|row| row.iter().any(|&x| x != zero))
            .count()
    }
}

