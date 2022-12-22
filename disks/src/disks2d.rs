use std::ops::Range;
use rand::Rng;

mod vec2;

use simulations_base::{Energy, MetropolisCriterion, MCProtocol, System};
use vec2::{Coordinates, square_grid_atoms, coordinates_to_pdb};

pub fn main() {
    const N: usize = 20;
    const R_REP: f64 = 4.0;
    const E_REP: f64 = 100.0;

    // ---------- system
    let mut system = Coordinates::new(N * N);
    system.set_box_len(N as f64 * 6.0);
    square_grid_atoms(&mut system);
    
    // ---------- Sampling
    let mut sampler: MCProtocol<MetropolisCriterion,Coordinates> =
        MCProtocol::new(MetropolisCriterion::new(1.0));
    sampler.add_mover(Box::new(single_atom_move), 0.1..3.0);

    // ---------- scoring
    let en: Box<dyn Energy<Coordinates>> = Box::new(HardDisk::new(R_REP,E_REP));
    
    // ---------- simulation
    println!("{}",en.energy(&system));
    coordinates_to_pdb(&system,1,"tra.pdb", false);
    for i in 0..1000 {
        sampler.make_sweeps(10,&mut system, &en);
        println!("{} {}", i, en.energy(&system));
        coordinates_to_pdb(&system,i+1,"tra.pdb", true);
    }
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
            if d2 > self.r2_2 { continue }
            if d2.le(&self.r2) { en_old += self.e_rep }
            d2 = new_system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { en_new += self.e_rep }
        }
        for j in pos+1..old_system.size() {
            let mut d2 = old_system.closest_distance_square(pos, j);
            if d2 > self.r2_2 { continue }
            if d2.le(&self.r2) { en_old += self.e_rep }
            d2 = new_system.closest_distance_square(pos, j);
            if d2.le(&self.r2) { en_new += self.e_rep }
        }
        (en_old, en_new)
    }
}

