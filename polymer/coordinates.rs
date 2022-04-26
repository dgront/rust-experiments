use rand::Rng;                      // to create a random versor

use std::io::stdout;
use std::io::{BufWriter,Write};
use std::fs::{File};

pub struct Coordinates {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
}

impl Coordinates {
    pub fn new(n: usize) -> Coordinates {
        let mut x = Vec::with_capacity(n);
        x.resize(n, 0.0);
        let mut y = Vec::with_capacity(n);
        y.resize(n, 0.0);
        let mut z = Vec::with_capacity(n);
        z.resize(n, 0.0);

        return Coordinates { x: x, y: y, z: z };
    }

    pub fn size(&self) -> usize { return self.x.len(); }

    pub fn set(&mut self, i:usize, v:f32) { self.x[i] = v; self.z[i] = v; self.y[i] = v;}

    pub fn distance_square(&self, i: usize, j: usize) -> f32 {
        let mut d = self.x[i] - self.x[j];
        let mut d2 = d * d;
        d = self.y[i] - self.y[j];
        d2 += d * d;
        d = self.z[i] - self.z[j];
        d2 += d * d;
        return d2;
    }

    pub fn to_pdb(&self, out_fname: & str) {

        let mut out_writer= BufWriter::new(
            if out_fname=="" {Box::new(stdout())}
            else {Box::new(stdout())});

        out_writer.write(b"MODEL    0\n");
        for i in 0..self.size() {
            out_writer.write(format!("ATOM   {:4}{}  ALA A{:4}    {:8.3}{:8.3}{:8.3}  1.00 99.88           C\n",
                    i+1, " CA ", i+1, self.x[i], self.y[i], self.z[i]).as_bytes());
        }
        out_writer.write(b"ENDMDL\n");
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


pub fn main() {
    let mut chain = Coordinates::new(30);
    chain.to_pdb("");
}