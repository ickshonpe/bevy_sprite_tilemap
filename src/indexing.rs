pub trait IndexableGrid {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    #[inline]
    fn index_grid(&self, x: usize, y: usize) -> usize {
        y * self.width() + x
    }

    #[inline]
    fn index_grid_checked(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width() && y < self.height() {
            Some(self.index_grid(x, y))
        } else {
            None
        }
    }
}
