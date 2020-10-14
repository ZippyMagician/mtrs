/// Is implemented for both `usize` and `(usize, usize)`, which allows for a more flexible input
/// when creating matrices
pub trait Size {
    fn height(&self) -> usize;

    fn width(&self) -> usize;

    fn dimensions(&self) -> (usize, usize) {
        (self.height(), self.width())
    }
}

impl Size for usize {
    fn height(&self) -> usize {
        *self
    }

    fn width(&self) -> usize {
        *self
    }
}

impl Size for (usize, usize) {
    fn height(&self) -> usize {
        self.0
    }

    fn width(&self) -> usize {
        self.1
    }
}
