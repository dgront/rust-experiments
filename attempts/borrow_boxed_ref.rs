use std::any::Any;

pub trait Observe {
    fn observe(&mut self, val: &f32);

    fn as_any(&self) -> &dyn Any;
}

struct Temperature { val: f32 }

impl Observe for Temperature {
    fn observe(&mut self, val: &f32) {
        self.val = *val;
        println!("{}", self.val);
    }

    fn as_any(&self) -> &dyn Any { self }
}

// ---------- MWE of a sampler
struct Sampler<'a> {
    pub observers: Vec<&'a mut Box<dyn Observe> >,
}


impl Sampler<'_> {
    pub fn run(&mut self, n:usize) {
        for k in 0..n {
            for i in 0..self.observers.len() {
                let o: &mut Box<dyn Observe> = self.observers[i];
                o.observe(&(k as f32));
            }
        }
    }
}

pub fn main() {

    let mut sampler: Sampler = Sampler{observers: Vec::new()};
    let temp: Temperature = Temperature{val:0.0};
    // --- after that call we no longer own `temp`
    let mut boxed_temp : Box<dyn Observe> = Box::new(temp);
    sampler.observers.push( &mut boxed_temp );
    // --- but here we still own boxed_temp, because `sampler` only borowed it

    // ---------- make 5 runs
    for _ in 0..5 {
        sampler.run(10);
    }
    // ---------- Let's recover a reference to the original Temperature struct
    let access_temp: &Temperature = match boxed_temp.as_any().downcast_ref::<Temperature>() {
        Some(b) => b,
        None => panic!("&a isn't a Temperature!"),
    };
    println!("Now the temperature is: {}", &access_temp.val);

}
