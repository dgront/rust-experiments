use crate::vec2::Coordinates;

pub trait Energy {
    fn energy(&self, system: &Coordinates) -> f64;
    fn energy_by_pos(&self, system: &Coordinates, pos: usize) -> f64;
}