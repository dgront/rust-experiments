// ---------- Say, these are two systems' types
struct S<T> { v: T, }

type Si16 = S<i16>;
type Sf32 = S<f32>;

// ---------- MWE of a sampler
pub trait Sampler<T> {
    // type T;
    // fn energy(&self, system: &Self::T) -> f64;
    // fn run(&mut self, system: &mut Self::T, n_steps:i32) -> f64;

    fn energy(&self, system: &T) -> f64;
    fn run(&mut self, system: &mut T, n_steps:i32) -> f64;
}

pub struct MC<T> { system:T}

impl<T> Sampler<T> for MC<T> {
    fn energy(&self, system: &T) -> f64 { 0.0 }

    fn run(&mut self, system: &mut T, n_steps: i32) -> f64 { 0.0 }
}

pub type IntegerSampler = Sampler<Si16>;
pub type IntegerMC =  MC<Si16>;

pub fn main() {
    let s: Si16 = Si16 { v: 10 };

    let mc:IntegerMC  = IntegerMC{system:s};
    let si: Si16 = Si16 { v: 10 };
    mc.energy(&si);
}