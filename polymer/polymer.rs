pub mod coordinates;

use rand::Rng;

mod forcefields {

    pub struct SquareWellContact {
        pub repulsion_ends: f32,
        pub contact_starts: f32,
        pub contact_ends: f32,
        pub repulsion_energy: f32,
        pub contact_energy: f32
    }
}

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
        if d2 < $A2 { $en += 100.0; }
        else {
            if d2 < $B2 {
                $en += -1.0;
            }
        }
    }
}

pub fn energy_for_position(pos:usize, chain: & coordinates::Coordinates) -> f64 {
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

    for i in 0..chain.size() {
        pairwise_contact_kernel!(x,y,z,chain, i, A2,B2, en);
        // let mut d = chain.x[i] - X;
        // let mut d2 = d*d;
        // if d2 > B2 { continue; }
        // d = chain.y[i] - Y;
        // d2 += d*d;
        // if d2 > B2 { continue; }
        // d = chain.z[i] - Z;
        // d2 += d*d;
        // if d2 < A2 { en += 100.0; }
        // else {
        //     if d2 < B2 {
        //         en += -1.0;
        //     }
        // }
    }
    return en - 100.0;      // subtract self-repulsion
}

pub fn energy(chain: & coordinates::Coordinates) -> f64 {
    let mut en: f64 = 0.0;
    for i in 0..chain.size() {
        en += energy_for_position(i,chain);
    }
    return en / 2.0;
}

pub fn sample(chain: &mut coordinates::Coordinates, temp: f64, n_cycles: i32) -> i32 {
    let mut rng = rand::thread_rng();

    let step :f32 = 0.05;
    let mut succ = 0;
    for i in 0..n_cycles {
        for j in 0..chain.size() {
            let i_moved = rng.gen_range(0..chain.size());
            let en_before = energy_for_position(i_moved,chain);

            let dx : f32 = rng.gen_range(-step..step);
            let dy : f32 = rng.gen_range(-step..step);
            let dz : f32 = rng.gen_range(-step..step);

            chain.x[i_moved] += dx;
            chain.y[i_moved] += dy;
            chain.z[i_moved] += dz;
            let en_after = energy_for_position(i_moved,chain);
            if ! metropolis_criterion(temp, en_before, en_after) {
                chain.x[i_moved] -= dx;
                chain.y[i_moved] -= dy;
                chain.z[i_moved] -= dz;
            } else { succ+=1; }
        }
    }
    return succ;
}

pub fn randomize_chain(bond_length:f32, chain: &mut coordinates::Coordinates) {
    chain.set(0,0.0);

    for i in 1..chain.size() {
        let mut go_on:bool = true;
        while go_on {
            let (x, y, z) = coordinates::random_unit_versor();
            chain.x[i] = chain.x[i-1] + x*bond_length;
            chain.y[i] = chain.y[i-1] + y*bond_length;
            chain.z[i] = chain.z[i-1] + z*bond_length;
            go_on = false;
            for j in 0..i {
                if chain.distance_square(j,i) < A2 {
                    go_on = true;
                    break;
                }
            }
        }
    }
}

pub fn main() {
    let n_small :i32 = 100000;
    let n_big :i32 = 1000;
    let n_beads :i32 = 100;
    let mut chain = coordinates::Coordinates::new(n_beads as usize);
    randomize_chain(3.8, &mut chain);
    chain.to_pdb("");
    for i in 0..n_big {
        let n_succ = sample(&mut chain,2.5,n_small);
        println!("{}, {}",energy(&chain), (n_succ as f32)/((n_small*n_beads) as f32));
        chain.to_pdb("");
    }
}