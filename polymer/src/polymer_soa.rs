pub mod coordinates_soa;

use rand::Rng;
use std::env;
use std::time::Instant;

const A :f32 = 3.0;
const A2 :f32 = A*A;
const B :f32 = 4.0;
const B2 :f32 = B*B;
const K :f32 = 1.0;     // stiffness

pub fn metropolis_criterion(temp: f64, en_before: f64, en_after: f64) -> bool {
    if en_after < en_before { return true; }
    let mut rng = rand::thread_rng();
    return f64::exp(-(en_after - en_before) / temp) > rng.gen();
}

macro_rules! pairwise_contact_kernel {
    ($x:expr,$y:expr,$z:expr,$chain:expr,$i:expr,$A2:expr,$B2:expr,$en:expr)=>{
        let mut d = $chain.x[$i] - $x;
        let mut d2 = d*d;
        if d2 > $B2 { continue; }
        d = $chain.y[$i] - $y;
        d2 += d*d;
        if d2 > $B2 { continue; }
        d = $chain.z[$i] - $z;
        d2 += d*d;
        if d2 < $A2 { $en += 1000.0; }
        else {
            if d2 < $B2 {
                $en += -1.0;
            }
        }
    }
}

pub fn energy_for_position(pos:usize, chain: & coordinates_soa::Coordinates) -> f64 {
    let mut en: f64 = 0.0;

    let x:f32 = chain.x[pos];
    let y:f32 = chain.y[pos];
    let z:f32 = chain.z[pos];
    if pos > 0 {
        let d = chain.distance_square(pos-1,pos) - 3.8;
        en += (K*d*d) as f64;
    }
    if pos < chain.size()-1 {
        let d = chain.distance_square(pos,pos+1) - 3.8;
        en += (K*d*d) as f64;
    }

    if pos > 1 {
        for i in 0..pos - 1 {
            pairwise_contact_kernel!(x,y,z,chain, i, A2, B2, en);
        }
    }
    if pos < chain.size()-2 {
        for i in pos + 2..chain.size() {
            pairwise_contact_kernel!(x,y,z,chain, i, A2, B2, en);
        }
    }
    return en;
}

pub fn energy(chain: & coordinates_soa::Coordinates) -> f64 {
    let mut en: f64 = 0.0;
    for i in 0..chain.size() {
        en += energy_for_position(i,chain);
    }
    return en / 2.0;
}

pub fn sample(chain: &mut coordinates_soa::Coordinates, temp: f64, n_cycles: i32) -> i32 {
    let mut rng = rand::thread_rng();

    let step :f32 = 0.5;
    let mut succ = 0;
    for _i in 0..n_cycles {
        for _j in 0..chain.size() {
            let i_moved = rng.gen_range(0..chain.size());
            let en_before = energy_for_position(i_moved,chain);

            let dx : f32 = rng.gen_range(-step..step);
            let dy : f32 = rng.gen_range(-step..step);
            let dz : f32 = rng.gen_range(-step..step);

            add_point_coordinates!(chain, i_moved, dx, dy, dz);
            let en_after = energy_for_position(i_moved,chain);
            if ! metropolis_criterion(temp, en_before, en_after) {
                add_point_coordinates!(chain, i_moved, -dx, -dy, -dz);
            } else { succ+=1; }
        }
    }
    return succ;
}

pub fn randomize_chain(bond_length:f32, chain: &mut coordinates_soa::Coordinates) {
    chain.set(0,0.0);

    for i in 1..chain.size() {
        let mut go_on:bool = true;
        while go_on {
            let (x, y, z) = coordinates_soa::random_unit_versor();
            chain.x[i] = chain.x[i-1] + x*bond_length;
            chain.y[i] = chain.y[i-1] + y*bond_length;
            chain.z[i] = chain.z[i-1] + z*bond_length;
            go_on = false;
            for j in 0..i {
                if chain.distance_square(j,i) <= A2 {
                    go_on = true;
                    break;
                }
            }
        }
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let n_small :i32 = 1000;
    let n_beads: i32 = if args.len() > 1 { args[1].parse::<i32>().unwrap() } else { 100 };
    let temp: f64 = if args.len() > 2 { args[2].parse::<f64>().unwrap() } else { 1.0 };
    let n_big :i32 = if args.len() > 3 { args[3].parse::<i32>().unwrap() } else { 1000 };
    let mut chain = coordinates_soa::Coordinates::new(n_beads as usize);
    randomize_chain(3.8, &mut chain);
    chain.to_pdb("");
    let before = Instant::now();
    for i in 0..n_big {
        let n_succ = sample(&mut chain,temp,n_small);
        let (cx, cy, cz) = chain.cm();
        println!("En: {} : {}, {}, {} {} {}, {:.2?}", i, energy(&chain),
                 (n_succ as f32) / ((n_small * n_beads) as f32), cx, cy, cz, before.elapsed());
        if i % 10 == 0 { chain.to_pdb(""); }
    }
}