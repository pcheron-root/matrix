use crate::structures::vector::Vector;
use std::fmt::Debug;

pub struct Matrix<T, const N: usize> {
    rows: [Vector<T, N>; N],
}

impl<T, const N: usize> Matrix<T, N>
where
    T: Debug,
{
    pub fn new(rows: [Vector<T, N>; N]) -> Self {
        Self { rows }
    }

    pub fn print(&self) {
        for row in &self.rows {
            row.print();
        }
    }
}