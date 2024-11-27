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

impl<
        T: std::fmt::Debug + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign,
        const N: usize,
    > Vector<T, N>
where
    T: Copy,
{
    /// add will change the Vector
    pub fn add(&mut self, v: &Vector<T, N>) {
        for i in 0..N {
            self.data[i] += v.data[i];
        }
    }
    pub fn sub(&mut self, v: &Vector<T, N>) {
        for i in 0..N {
            self.data[i] -= v.data[i];
        }
    }
    pub fn scl(&mut self, x: T) {
        for i in 0..N {
            self.data[i] *= x;
        }
    }
}
