use std::ops::{Index, IndexMut};
use std::fs::File;
use std::io::{Write};

#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y}
    }
    pub fn from_float(value: f64) -> Vec2 {
        Vec2 {
            x: value,
            y: value
        }
    }
}

#[derive(Clone, Debug)]
pub struct Coordinates {
    box_len: f64,
    box_len_half: f64,
    v: Vec<Vec2>,
}

macro_rules! wrap_coordinate_to_box {
    ($val:expr, $L:expr, $coord:expr) => {
        $coord = $val;
        if $coord > $L { $coord = $coord - $L}
        else {
            if $coord < 0.0 { $coord = $L + $coord}
        }
    }
}

macro_rules! closest_image {
    ($c1:expr, $c2:expr, $L: expr,$L2: expr, $delta:expr) => {
        $delta = $c1 - $c2;
        if $delta > 0.0 {
            if $delta > $L2 {$delta -= $L}
        } else {
            if $delta < -$L2 {$delta += $L}
        }
    }
}

pub trait System: Clone {
    fn size(&self) -> usize;
    fn copy_from(&mut self, i:usize, rhs: &Self);
}

impl Coordinates {

    pub fn new(n: usize) -> Coordinates {
        let mut v = if n > 0 { Vec::with_capacity(n) } else { Vec::new() };

        if n > 0 {
            let zero = Vec2::from_float(0.0);
            v.resize(n, zero);
        }
        let l: f64 = 100000.0;
        return Coordinates {box_len: l, box_len_half: l/2.0, v};
    }

    #[inline(always)]
    pub fn box_len(&self) -> f64 { self.box_len }

    #[inline(always)]
    pub fn set_box_len(&mut self, new_box_len: f64) {
        self.box_len = new_box_len;
        self.box_len_half = new_box_len / 2.0;
    }

    pub fn distance_square(&self, i: usize, j: usize) -> f64 {

        let mut d = self.v[i].x - self.v[j].x;
        let mut d2 = d * d;
        d = self.v[i].y - self.v[j].y;
        d2 += d * d;
        return d2;
    }

    pub fn closest_distance_square(&self, i: usize, j: usize) -> f64 {

        let mut d:f64;
        closest_image!(self.v[i].x, self.v[j].x, self.box_len, self.box_len_half, d);
        let d2 = d * d;
        closest_image!(self.v[i].y, self.v[j].y, self.box_len, self.box_len_half, d);

        return d2 + d*d;
    }

    pub fn closest_distance_square_to_vec(&self, i: usize, v: &Vec2) -> f64 {

        let mut d:f64;
        closest_image!(self.v[i].x, v.x, self.box_len, self.box_len_half, d);
        let d2 = d * d;
        closest_image!(self.v[i].y, v.y, self.box_len, self.box_len_half, d);

        return d2 + d*d;
    }

    /// Calculates the difference in ``x`` coordinate between the i-th atom and a given ``x`` value
    /// This function obeys periodic boundary conditions and returns the distance to the closest
    /// image of the  position ``i``
    pub fn delta_x(&self, i: usize, x: f64) -> f64 {
        let mut d: f64;
        closest_image!(self.v[i].x,x, self.box_len, self.box_len_half, d);
        d
    }

    /// Calculates the difference in ``y`` coordinate between the i-th atom and a given ``y`` value
    /// This function obeys periodic boundary conditions and returns the distance to the closest
    /// image of the  position ``i``
    pub fn delta_y(&self, i: usize, y: f64) -> f64 {
        let mut d: f64;
        closest_image!(self.v[i].y, y, self.box_len, self.box_len_half, d);
        d
    }

    pub fn x(&self, i:usize) -> f64 { self.v[i].x }

    pub fn y(&self, i:usize) -> f64 { self.v[i].y }

    pub fn set_x(&mut self, i:usize, x: f64) {  wrap_coordinate_to_box!(x, self.box_len, self.v[i].x); }

    pub fn set_y(&mut self, i:usize, y: f64) {  wrap_coordinate_to_box!(y, self.box_len, self.v[i].y); }

    pub fn set(&mut self, i:usize, x: f64, y: f64) {
        wrap_coordinate_to_box!(x, self.box_len, self.v[i].x);
        wrap_coordinate_to_box!(y, self.box_len, self.v[i].y);
    }

    pub fn add(&mut self, i:usize, x: f64, y: f64) {
        wrap_coordinate_to_box!(self.v[i].x + x, self.box_len, self.v[i].x);
        wrap_coordinate_to_box!(self.v[i].y + y, self.box_len, self.v[i].y);
    }

}

impl System for Coordinates {

    fn size(&self) -> usize { return self.v.len(); }

    /// Copy coordinates of i-th atom from a given rhs coordinates
    /// This method (unlike set()) does not apply PBC. To the contrary, it assumes the two systems:
    /// this and RHS have exactly the same simulation box geometry
    fn copy_from(&mut self, i:usize, rhs: &Coordinates) {
        self.v[i].x = rhs.v[i].x;
        self.v[i].y = rhs.v[i].y;
    }
}

impl Index<usize> for Coordinates {
    type Output = Vec2;
    fn index(&self, i: usize) -> &Vec2 {
        &self.v[i]
    }
}

impl IndexMut<usize> for Coordinates {
    fn index_mut(&mut self, i: usize) -> &mut Vec2 {
        &mut self.v[i]
    }
}

pub fn square_grid_atoms(system: &mut Coordinates) {

    let points_one_side: usize = (f64::powf(system.size() as f64, 0.5)).ceil() as usize;
    let dw = system.box_len() / points_one_side as f64;
    let cell_margin = dw / 2.0;

    for i in 0..system.size() {
        let k = i % points_one_side;
        let l = i / points_one_side;
        system.set(i,dw * k as f64 + cell_margin,dw * l as f64 + cell_margin);
    }
}

pub fn coordinates_to_pdb(chain: &Coordinates, i_model: i16, out_fname: &str, if_append: bool) {
    let mut out_writer = File::options().append(if_append).write(true).create(true).open(&out_fname).ok().unwrap();

    out_writer.write(format!("MODEL    {i_model}\n").as_bytes()).ok();
    for i in 0..chain.size() {
        out_writer.write(format!("ATOM   {:4}{}  ALA A{:4}    {:8.3}{:8.3}{:8.3}  1.00 99.88           C\n",
                                 i+1, " CA ", i+1, chain.x(i), chain.y(i), 0.0).as_bytes()).ok();
    }
    out_writer.write(b"ENDMDL\n").ok();
}