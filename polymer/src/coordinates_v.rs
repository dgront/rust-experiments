use crate::vec3::Vec3;

use rand::Rng;                      // to create a random versor

use std::io::stdout;
use std::io::{BufWriter,Write};
use std::fs::{File};

pub struct CoordinatesV {
    pub v: Vec<Vec3>
}

impl CoordinatesV {
    pub fn new(n: usize) -> CoordinatesV {
        let mut v = Vec::with_capacity(n);
        let mut zero = Vec3::from_float(0.0);
        v.resize(n, zero);

        return CoordinatesV { v};
    }

    pub fn size(&self) -> usize { return self.v.len(); }

    pub fn set(&mut self, i:usize, v:f32) { self.v[i].x = v; self.v[i].z = v; self.v[i].y = v;}

    pub fn distance_square(&self, i: usize, j: usize) -> f32 {
        let mut d = self.v[i].x - self.v[j].x;
        let mut d2 = d * d;
        d = self.v[i].y - self.v[j].y;
        d2 += d * d;
        d = self.v[i].z - self.v[j].z;
        d2 += d * d;
        return d2;
    }

    pub fn to_pdb(&self, out_fname: & str) {

        let mut out_writer= BufWriter::new(
            if out_fname=="" {Box::new(stdout())}
            else {Box::new(stdout())});

        out_writer.write(b"MODEL    0\n").ok();
        for i in 0..self.size() {
            out_writer.write(format!("ATOM   {:4}{}  ALA A{:4}    {:8.3}{:8.3}{:8.3}  1.00 99.88           C\n",
                                     i+1, " CA ", i+1, self.v[i].x, self.v[i].y, self.v[i].z).as_bytes()).ok();
        }
        out_writer.write(b"ENDMDL\n").ok();
    }

    pub fn cm(&self) -> (f64,f64,f64) {
        let mut cx :f64 = 0.0;
        let mut cy :f64 = 0.0;
        let mut cz :f64 = 0.0;
        for i in 0..self.size() { cx += self.v[i].x as f64; }
        for i in 0..self.size() { cy += self.v[i].y as f64; }
        for i in 0..self.size() { cz += self.v[i].z as f64; }
        let n: f64 = self.size() as f64;
        return (cx / n, cy / n, cz / n);
    }
}

pub fn random_unit_versor() -> (f32, f32, f32) {

    let mut rng = rand::thread_rng();
    let x : f32 = rng.gen_range(-1.0..1.0);
    let y : f32 = rng.gen_range(-1.0..1.0);
    let z : f32 = rng.gen_range(-1.0..1.0);
    let l =  { (x * x + y * y + z * z).sqrt() };
    return (x/l, y/l, z/l);
}

#[macro_export]
macro_rules! add_point_coordinates_v {
    ($coordinates:expr,$i_pos:expr,$x:expr,$y:expr,$z:expr)=>{
        $coordinates.v[$i_pos].x += $x;
        $coordinates.v[$i_pos].y += $y;
        $coordinates.v[$i_pos].z += $z;
    }
}

#[macro_export]
macro_rules! set_point_coordinates_v {
    ($coordinates:expr,$i_pos:expr,$x:expr,$y:expr,$z:expr)=>{
        $coordinates.v[$i_pos].x = $x;
        $coordinates.v[$i_pos].y = $y;
        $coordinates.v[$i_pos].z = $z;
    }
}
