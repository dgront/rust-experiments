use std::ops::Range;
use rand::Rng;

mod vec2;

use simulations_base::{Energy, MetropolisCriterion, MCProtocol, System, Mover, AcceptanceStatistics,
                       MoversSet, AdaptiveMCProtocol, Sampler};
use vec2::{Coordinates, square_grid_atoms, coordinates_to_pdb};

pub fn main() {
    const N: usize = 20;
    const R_REP: f64 = 4.0;
    const E_REP: f64 = 100000.0;

    // ---------- system
    let mut system = Coordinates::new(N * N);
    system.set_box_len(N as f64 * 6.0);
    square_grid_atoms(&mut system);
    
    // ---------- Sampling
    let mut simple_sampler: MCProtocol<MetropolisCriterion,Coordinates> =
        MCProtocol::new(MetropolisCriterion::new(1.0));
    simple_sampler.add_mover(Box::new(DiskMover::new(3.0)));

    let mut sampler = AdaptiveMCProtocol::new(Box::new(simple_sampler));
    sampler.target_rate = 0.2;

    // ---------- scoring
    let en: Box<dyn Energy<Coordinates>> = Box::new(HardDisk::new(R_REP,E_REP));

    // ---------- observers
    let mut density: ObserveDensity = ObserveDensity::new(1.0, 20 * 6);

    // ---------- simulation
    println!("{}",en.energy(&system));
    let mut recent_acceptance = AcceptanceStatistics::default();
    coordinates_to_pdb(&system,1,"tra.pdb", false);
    for i in 0..10000 {
        sampler.make_sweeps(100,&mut system, &en);
        let stats = sampler.get_mover(0).acceptance_statistics();
        println!("{} {} {}", i, en.energy(&system),
                 stats.recent_success_rate(&recent_acceptance));
        recent_acceptance = stats;
        coordinates_to_pdb(&system,i+1,"tra.pdb", true);
        density.observe(&system);
    }

    density.close();
}

struct DiskMover {
    max_step: f64,
    succ_rate: AcceptanceStatistics
}

impl DiskMover {
    pub fn new(max_range: f64) -> DiskMover {
        DiskMover{ max_step: max_range, succ_rate: Default::default() }
    }
}

impl Mover<Coordinates> for DiskMover {

    fn perturb(&mut self, system: &mut Coordinates) -> Range<usize> {
        let mut rng = rand::thread_rng();
        let i_moved = rng.gen_range(0..system.size());
        system.add(i_moved,rng.gen_range(-self.max_step..self.max_step),
                   rng.gen_range(-self.max_step..self.max_step));

        i_moved..i_moved
    }

    fn acceptance_statistics(&self) -> AcceptanceStatistics { self.succ_rate.clone() }

    fn add_success(&mut self) { self.succ_rate.n_succ += 1; }

    fn add_failure(&mut self) { self.succ_rate.n_failed += 1; }

    fn max_range(&self) -> f64 { return self.max_step; }

    fn set_max_range(&mut self, new_val: f64) { self.max_step = new_val; }
}

pub fn single_atom_move(future: &mut Coordinates, max_step:f64) -> Range<usize> {
    let mut rng = rand::thread_rng();
    let i_moved = rng.gen_range(0..future.size());
    future.add(i_moved,rng.gen_range(-max_step..max_step),
               rng.gen_range(-max_step..max_step));

    i_moved..i_moved
}

struct HardDisk { r: f64, e_rep: f64, r2: f64, r2_2: f64 }

impl HardDisk {
    pub fn new(r:f64, e_rep: f64) -> HardDisk {
        HardDisk { r, e_rep, r2: r * r, r2_2: 4.0 * r * r }
    }

    pub fn r(&self) -> f64 { self.r }
}

impl Energy<Coordinates> for HardDisk {

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

    fn delta_energy_by_pos(&self, old_system: &Coordinates, new_system: &Coordinates, pos: usize) -> (f64, f64) {
        let (mut en_old, mut en_new) = (0.0f64, 0.0f64);
        for j in 0..pos {
            let mut d2 = old_system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { en_old += self.e_rep }
            d2 = new_system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { en_new += self.e_rep }
        }
        for j in pos+1..old_system.size() {
            let mut d2 = old_system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { en_old += self.e_rep }
            d2 = new_system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { en_new += self.e_rep }
        }
        (en_old, en_new)
    }
}

pub struct ObserveDensity {
    m:Vec<Vec<u32>>,
    dxy: f64
}

impl ObserveDensity {
    pub fn new(dxy:f64, nxy:u16) -> ObserveDensity {
        let m = vec![vec![0; nxy as usize]; nxy as usize];
        ObserveDensity{m, dxy}
    }

    pub fn observe(&mut self, system: &Coordinates) {
        for i in 0..system.size() {
            let x = (system.x(i) / self.dxy) as usize;
            let y = (system.y(i) / self.dxy) as usize;
            self.m[x][y] += 1;
        }
    }

    pub fn close(&mut self) {
        for i in 0..self.m.len() {
            for j in 0..self.m.len() {
                println!("{} {} {}", i, j, self.m[i][j]);
            }
        }
    }
}