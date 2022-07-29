pub trait Observe {

    fn observe(&mut self, val: &f32);
}

struct Temperature { val: f32 }

impl Observe for Temperature {
    fn observe(&mut self, val: &f32) {
        self.val = *val;
        println!("{}", self.val);
    }
}

// ---------- MWE of a sampler
struct Sampler {
    pub observers: Vec<Box<dyn Observe>>,
}


impl Sampler {
    pub fn run(&mut self, n:usize) {
        for k in 0..n {
            for i in 0..self.observers.len() {
                self.observers[i].observe(&(k as f32));
            }
        }
    }
}

pub fn main() {

    let mut sampler: Sampler = Sampler{observers: Vec::new()};
    let mut temp: Temperature = Temperature{val:0.0};
    sampler.observers.push( Box::new(temp) );

    sampler.run(10);
}