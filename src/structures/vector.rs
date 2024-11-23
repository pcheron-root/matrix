
// use std::fmt::Debug;

pub struct Vector<T, const N: usize> {
    components: [T; N],
}

impl<T: std::fmt::Debug, const N: usize> Vector<T, N> {
    pub fn new(components: [T; N]) -> Self {
        Self { components }
    }

    pub fn size(&self) -> usize {
        N
    }

    pub fn print(&self) {
        println!("{:?}", self.components);
    }
}