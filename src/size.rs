/// Is implemented for both `usize` and `(usize, usize)`, which allows for a more flexible input
/// when creating matrices
pub trait Size {
    fn dim(&self) -> (usize, usize);
}

impl Size for usize {
    fn dim(&self) -> (usize, usize) {
        (*self, *self)
    }
}

impl Size for (usize, usize) {
    fn dim(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}
