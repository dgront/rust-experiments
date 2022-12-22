
use std::ops::Range;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::Energy;
use crate::{System};

pub struct AdaptiveMoverStats {
    n_succ:i32,
    n_failed:i32,
    move_range: f64,
    factor: f64,
    max_move_range:Range<f64>
}

impl AdaptiveMoverStats {
    pub fn new(max_move_range:Range<f64>) -> AdaptiveMoverStats {
        AdaptiveMoverStats {
            n_succ: 0,
            n_failed: 0,
            move_range: (max_move_range.end - max_move_range.start)/2.0,
            factor: 0.95,
            max_move_range: max_move_range,
        }
    }

    pub fn add_success(&mut self) { self.n_succ+=1; }
    pub fn add_failure(&mut self) { self.n_failed+=1; }
    pub fn success_rate(&self) -> f64 { self.n_succ as f64 / (self.n_succ as f64 + self.n_failed as f64) }

    pub fn adapt_range(&mut self) {
        let rate = self.success_rate();
        if rate < 0.35 { self.move_range *= self.factor }
        if rate > 0.45 { self.move_range /=self.factor }
        if self.max_move_range.end.lt(&self.move_range) { self.move_range = self.max_move_range.end }
        if self.max_move_range.start.gt(&self.move_range) { self.move_range = self.max_move_range.start }
    }
}

pub trait AcceptanceCriterion {
    fn check(&mut self, energy_before: f64, energy_after: f64) -> bool;
}

pub struct MetropolisCriterion {
    pub temperature: f64,
    rng: ThreadRng
}

impl MetropolisCriterion {
    pub fn new(temperature: f64) -> MetropolisCriterion { MetropolisCriterion{temperature, rng:rand::thread_rng()} }
}

impl AcceptanceCriterion for MetropolisCriterion {
    fn check(&mut self, energy_before: f64, energy_after: f64) -> bool {
        // let mut rng = rand::thread_rng();
        let delta_e = energy_after - energy_before;
        if delta_e <= 0.0 || self.rng.gen_range(0.0..1.0) <= (-delta_e / self.temperature).exp() { return true }
        return false;
    }
}

pub struct MCProtocol<T: AcceptanceCriterion, S: System> {
    pub acceptance_criterion: T,
    movers_stats: Vec<AdaptiveMoverStats>,
    movers: Vec<Box<dyn Fn(&mut S,f64) -> Range<usize>>>
}


impl<T: AcceptanceCriterion, S: System> MCProtocol<T, S> {
    pub fn new(acc_crit: T) -> MCProtocol<T, S> {
        MCProtocol {
            acceptance_criterion: acc_crit,
            movers_stats: vec![],
            movers: vec![]
        }
    }

    pub fn add_mover(&mut self, perturb_fn: Box<dyn Fn(&mut S,f64) -> Range<usize>>, allowed_range: Range<f64>){
        self.movers_stats.push(AdaptiveMoverStats::new(allowed_range));
        self.movers.push(perturb_fn);
    }

    pub fn make_sweeps(&mut self, n:usize, coords: &mut S, energy: &Box<dyn Energy<S>>) {
        for _ in 0..n {
            self.make_sweep(coords, energy);
        }
    }

    pub fn make_sweep(&mut self, coords: &mut S, energy: &Box<dyn Energy<S>>) {
        let mut future_coords = coords.clone();
        for i_mover in 0..self.movers.len() {
            let stats = &mut self.movers_stats[i_mover];
            let mover = & self.movers[i_mover];
            for _ in 0..coords.size() {
                // ---------- Make a move on future system
                let range: Range<usize> = mover(&mut future_coords, stats.move_range);
                // ---------- Evaluate energy difference
                let (en_before, en_after) = energy.delta_energy_by_pos(coords, &future_coords, range.start);
                // ---------- apply acceptance criterion, copy or undo the move
                if self.acceptance_criterion.check(en_before, en_after) {
                    // --- update mover counts, copy future_pose on current_pose to make the move
                    for ipos in range.start..range.end + 1 {
                        coords.copy_from(ipos, &future_coords);
                    }
                    stats.add_success();
                }
                else {
                    // --- update mover failures, copy current_pose on future_pose to clear the move
                    for ipos in range.start..range.end + 1 {
                        future_coords.copy_from(ipos, &coords);
                    }
                    stats.add_failure();
                }
            }
        }
    }
}
