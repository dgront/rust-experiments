
pub trait Energy<S> {
    fn energy(&self, system: &S) -> f64;
    fn energy_by_pos(&self, system: &S, pos: usize) -> f64;
    fn delta_energy_by_pos(&self, old_system: &S, new_system: &S, pos: usize) -> (f64, f64);
}