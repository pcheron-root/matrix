
pub struct Vector<K> {
    pub x : K,
    }

impl<K> Vector<K> {
    pub fn new(x: K) -> Self {
        Self { x }
    }
}
