use std::fmt::Debug;

pub struct Matrix<K, const M: usize, const N: usize> {
    data: [[K; N]; M],
}

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
