use rand::Rng;
use std::collections::HashMap;
use std::ops::Range;

use bioshell_core::sequence::Sequence;

use simulations_base::{Energy, MetropolisCriterion, MCProtocol, System};

#[derive(Clone)]
pub struct SequenceSystem (Vec<u8>);

impl System for SequenceSystem {
    fn size(&self) -> usize { self.0.len() }

    fn copy_from(&mut self, i: usize, rhs: &Self) { self.0[i] = rhs.0[i]; }
}

pub struct Couplings {
    pub n: usize,
    pub k: usize,
    cplngs: Vec<Vec<f32>>,
    index_to_aa: Vec<u8>,
    aa_to_index: HashMap<u8, usize>,
}

impl Couplings {
    /// Creates an empty Coupling instance i.e. none of amino acids are coupled
    pub fn new(seq_len: usize, aa_order: &str) -> Couplings {
        let m = vec![vec![0.0; seq_len * aa_order.len()]; seq_len * aa_order.len()];
        let index_to_aa = aa_order.as_bytes().to_vec();
        println!("amino acid order: {:?}",index_to_aa);
        let mut aa_to_index = HashMap::new();
        for i in 0..aa_order.len() {
            aa_to_index.insert(index_to_aa[i],i);
        }
        let mut out = Couplings { n: seq_len, k: aa_order.len(), cplngs: m, index_to_aa, aa_to_index };
        out.init_couplings_diagonaly();
        return out
    }

    /// Initializes coupling diagonally.
    ///
    pub fn init_couplings_diagonaly(&mut self) {
        for i in 1..self.n {
            let ii = i * self.k;
            for j in 0..self.k {
                self.cplngs[ii + j][ii - self.k + j] = -1.0;
                self.cplngs[ii - self.k + j][ii + j] = -1.0;
            }
        }
        for j in 0..self.k {
            self.cplngs[j][self.k + j] = -1.0;
            self.cplngs[self.k + j][j] = -1.0;
        }
    }

    /// Prints the large matrix of couplings on the screen
    pub fn show_matrix(n:usize, k:usize, m: &Vec<Vec<f32>>) {
        for (i, row) in m.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                print!(" {:.3}",val);
                if j % k == (k - 1) { print!(" ") }
            }
            println!("");
            if i % k == (k - 1) { println!("#") }
        }
    }

    /// Prints the large matrix of couplings on the screen
    pub fn show(&self) { Couplings::show_matrix(self.n, self.k, &self.cplngs); }

    pub fn decode_sequence(&self, system: &Vec<u8>) -> String {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.reserve(system.len());
        for i in 0..system.len() {
            buffer.push(self.index_to_aa[system[i] as usize]);
        }
        String::from_utf8_lossy(&buffer).to_string()
    }

    pub fn delta_energy(&self, system: &Vec<u8>, pos: usize, old: usize, new: usize) -> f32 {
        let mut en: f32 = 0.0;
        let mut pos_j: usize = 0;
        let pos_i: usize = pos * self.k;
        for aa_j in system.iter() {
            en += self.cplngs[pos_j + *aa_j as usize][pos_i + new] - self.cplngs[pos_j + *aa_j as usize][pos_i + old];
            // println!("{} {} {} {} {}", pos_j, aa_j, cplngs[pos_j + *aa_j as usize][pos_i + new], cplngs[pos_j + *aa_j as usize][pos_i + old], en);
            pos_j+=self.k;
        }

        return en;
    }
}

impl Energy<SequenceSystem> for Couplings {
    fn energy(&self, system: &SequenceSystem) -> f64 {
        let mut en:f64 = 0.0;
        let mut pos_i :usize = 0;
        for aa_i in system.0.iter() {
            let mut pos_j :usize = 0;
            for aa_j in system.0.iter() {
                en += self.cplngs[pos_i + *aa_i as usize][pos_j + *aa_j as usize] as f64;
                pos_j += self.k;
            }
            pos_i += self.k;
        }
        return en/2.0;
    }

    fn energy_by_pos(&self, system: &SequenceSystem, pos: usize) -> f64 {
        let mut en: f64 = 0.0;
        let mut pos_j: usize = 0;
        let pos_i: usize = pos * self.k;
        let aa_i = system.0[pos];
        for aa_j in system.0.iter() {
            en += self.cplngs[pos_j + *aa_j as usize][pos_i + aa_i as usize] as f64;
            // println!("{} {} {} {} {}", pos_j, aa_j, cplngs[pos_j + *aa_j as usize][pos_i + new], cplngs[pos_j + *aa_j as usize][pos_i + old], en);
            pos_j+=self.k;
        }

        return en;
    }

    fn delta_energy_by_pos(&self, old_system: &SequenceSystem, new_system: &SequenceSystem, pos: usize) -> (f64, f64) {
        let (mut en_old, mut en_new) = (0.0, 0.0);
        let mut pos_j: usize = 0;
        let pos_i: usize = pos * self.k;
        let aa_i_old = old_system.0[pos];
        let aa_i_new = new_system.0[pos];
        for aa_j in old_system.0.iter() {
            en_new += self.cplngs[pos_j + *aa_j as usize][pos_i + aa_i_new as usize] as f64;
            en_old += self.cplngs[pos_j + *aa_j as usize][pos_i + aa_i_old as usize] as f64;
            // println!("{} {} {} {} {}", pos_j, aa_j, cplngs[pos_j + *aa_j as usize][pos_i + new], cplngs[pos_j + *aa_j as usize][pos_i + old], en);
            pos_j+=self.k;
        }

        return (en_old, en_new);
    }
}

pub fn accumulate_counts(system: &SequenceSystem, n_aa:usize, counts: &mut Vec<Vec<f32>>) {
    let mut pos_i: usize = 0;
    for aa_i in system.0.iter() {
        let mut pos_j: usize = 0;
        for aa_j in system.0.iter() {
            counts[pos_i + *aa_i as usize][pos_j + *aa_j as usize] += 1.0;
            pos_j += n_aa;
        }
        pos_i += n_aa;
    }
}

// pub fn isothermal_mc(system: &mut Vec<u8>, energy: &Couplings, inner_cycles: i32, outer_cycles: i32) -> Vec<Vec<f32>>{
//
//     let mut counts: Vec<Vec<f32>> = vec![vec![0.0; energy.n * energy.k]; energy.n * energy.k];
//     let mut rng = rand::thread_rng();
//     let mut total_en:f32 = energy.total_energy(&system);
//     let mut n_obs: f32 = 0.0;
//     let mut n_succ: f32 = 0.0;
//     for io in 0..outer_cycles {
//         for ii in 0..inner_cycles {
//             for is in 0..system.len() {
//                 let pos: usize = rng.gen_range(0..energy.n);
//                 let new_aa: usize = rng.gen_range(0..energy.k);
//                 let delta_en = energy.delta_energy(system, pos, system[pos] as usize, new_aa);
//                 if delta_en > 0.0 && (-delta_en).exp() < rng.gen_range(0.0..1.0) { continue; }
//                 system[pos] = new_aa as u8;
//                 total_en += delta_en;
//                 n_succ += 1.0;
//             }
//         }
//         n_obs += 1.0;
//         accumulate_counts(&system, energy.k, &mut counts);
//         println!("{:4} {}",total_en, energy.decode_sequence(&system));
//     }
//     println!("#{}",n_succ / (outer_cycles*inner_cycles*system.len() as i32) as f32);
//     counts.iter_mut().for_each(|el| el.iter_mut().for_each(|iel| *iel /= n_obs));
//     return counts;
// }

pub fn single_aa_move(future: &mut SequenceSystem, max_step:f64) -> Range<usize> {
    let mut rng = rand::thread_rng();
    let i_moved = rng.gen_range(0..future.size());

    future.0[i_moved] = rng.gen_range(0..max_step as u8);

    i_moved..i_moved
}

pub fn main() {
    // ---------- The system under study
    let seq_len: usize = 56;
    let mut system = SequenceSystem(vec![0; seq_len]);

    // ---------- Coupling energy
    let aa_order = "ACDEFGHIKLMNPQRSTVWY-";
    let aa_len = aa_order.len();
    let mut en: Box<dyn Energy<SequenceSystem>> = Box::new(Couplings::new(seq_len, aa_order));
    en.energy(&system);

    // ---------- Sampling
    let mut sampler: MCProtocol<MetropolisCriterion,SequenceSystem> = MCProtocol::new(MetropolisCriterion::new(1.0));
    sampler.add_mover(Box::new(single_aa_move), 0.0..aa_len as f64);

    // ---------- Observe counts for amino acids
    let mut counts: Vec<Vec<f32>> = vec![vec![0.0; seq_len * aa_len]; seq_len * aa_len];

    for i in 0..1000 {
        sampler.make_sweeps(1000,&mut system, &en);
        accumulate_counts(&system, aa_len, &mut counts);
        println!("{} {}", i, en.energy(&system));
    }

    // let counts = isothermal_mc(&mut system, &en,1000,10000);
    // en.show();
    // Couplings::show_matrix(en.n, en.k, &counts);
}