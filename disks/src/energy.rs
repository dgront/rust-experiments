// use crate::vec2::Coordinates;

pub trait Energy<S> {
    fn energy(&self, system: &S) -> f64;
    fn energy_by_pos(&self, system: &S, pos: usize) -> f64;
}