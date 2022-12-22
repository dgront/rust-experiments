pub trait System: Clone {
    fn size(&self) -> usize;
    fn copy_from(&mut self, i:usize, rhs: &Self);
}