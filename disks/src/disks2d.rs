use std::ops::Range;
use rand::Rng;

mod vec2;
use vec2::{Coordinates, square_grid_atoms};
use crate::vec2::coordinates_to_pdb;

pub fn main() {
    let n: usize = 20;
    // ---------- system
    let mut system = Coordinates::new(n*n);
    system.set_box_len(n as f64 * 6.0);
    square_grid_atoms(&mut system);
    
    // ---------- Sampling
    let mut sampler: MCProtocol<MetropolisCriterion> = MCProtocol::new(MetropolisCriterion{ temperature: 1.0});
    sampler.add_mover(Box::new(single_atom_move), 0.1..3.0);

    // ---------- scoring
    let en: Box<dyn Energy> = Box::new(HardDisk::new(4.0,100.0));
    
    // ---------- simulation
    println!("{}",en.energy(&system));
    coordinates_to_pdb(&system,1,"tra.pdb", false);
    for i in 0..1000 {
        sampler.make_sweeps(10,&mut system, &en);
        println!("{} {}", i, en.energy(&system));
        coordinates_to_pdb(&system,i+1,"tra.pdb", true);
    }
}

pub trait Energy {
    fn energy(&self, system: &Coordinates) -> f64;
    fn energy_by_pos(&self, system: &Coordinates, pos: usize) -> f64;
}

struct HardDisk { r: f64, e_rep: f64, r2: f64 }

impl HardDisk {
    pub fn new(r:f64, e_rep: f64) -> HardDisk {
        HardDisk{ r, e_rep, r2: r*r }
    }

    pub fn r(&self) -> f64 { self.r }
}

impl Energy for HardDisk {

    fn energy(&self, system: &Coordinates) -> f64 {
        let mut e = 0.0f64;
        for i in 1..system.size() {
            for j in 0..i {
                let d2 = system.closest_distance_square(i, j);
                if d2.le(&self.r2) { e += self.e_rep }
            }
        }
        return e;
    }

    fn energy_by_pos(&self, system: &Coordinates, pos: usize) -> f64 {
        let mut e = 0.0f64;
        for j in 0..pos {
            let d2 = system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { e += self.e_rep }
        }
        for j in pos+1..system.size() {
            let d2 = system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { e += self.e_rep }
        }
        return e;
    }
}

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

pub fn single_atom_move(future: &mut Coordinates, max_step:f64) -> Range<usize> {
    let mut rng = rand::thread_rng();
    let i_moved = rng.gen_range(0..future.size());
    future.add(i_moved,rng.gen_range(-max_step..max_step),
               rng.gen_range(-max_step..max_step));

    i_moved..i_moved
}

pub trait AcceptanceCriterion {
    fn check(&mut self, energy_before: f64, energy_after: f64) -> bool;
}

struct MetropolisCriterion {
    pub temperature: f64
}

impl AcceptanceCriterion for MetropolisCriterion {
    fn check(&mut self, energy_before: f64, energy_after: f64) -> bool {
        let mut rng = rand::thread_rng();
        let delta_e = energy_after - energy_before;
        if delta_e <= 0.0 || rng.gen_range(0.0..1.0) <= (-delta_e / self.temperature).exp() { return true }
        return false;
    }
}

struct MCProtocol<T: AcceptanceCriterion> {
    pub acceptance_criterion: T,
    movers_stats: Vec<AdaptiveMoverStats>,
    movers: Vec<Box<dyn Fn(&mut Coordinates,f64) -> Range<usize>>>
}


impl<T: AcceptanceCriterion> MCProtocol<T> {
    pub fn new(acc_crit: T) -> MCProtocol<T> {
        MCProtocol {
            acceptance_criterion: acc_crit,
            movers_stats: vec![],
            movers: vec![]
        }
    }

    pub fn add_mover(&mut self, perturb_fn: Box<dyn Fn(&mut Coordinates,f64) -> Range<usize>>, allowed_range: Range<f64>){
        self.movers_stats.push(AdaptiveMoverStats::new(allowed_range));
        self.movers.push(perturb_fn);
    }

    pub fn make_sweeps(&mut self, n:usize, coords: &mut Coordinates, energy: &Box<dyn Energy>) {
        for _ in 0..n {
            self.make_sweep(coords, energy);
        }
    }

    pub fn make_sweep(&mut self, coords: &mut Coordinates, energy: &Box<dyn Energy>) {
        let mut future_coords = coords.clone();
        for i_mover in 0..self.movers.len() {
            let stats = &mut self.movers_stats[i_mover];
            let mover = & self.movers[i_mover];
            for _ in 0..coords.size() {
                // ---------- Make a move on future system
                let range: Range<usize> = mover(&mut future_coords, stats.move_range);
                // ---------- Evaluate energy
                let en_before = energy.energy_by_pos(coords,range.start);
                let en_after = energy.energy_by_pos(&future_coords, range.start);
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

