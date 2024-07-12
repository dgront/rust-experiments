struct S {
    v: i32
}

impl S {
    pub fn get_v(&self) -> i32 { self.v }
}

fn main() {
    let s = S{v:10};
    // s.get_v();
    println!("{}", S::get_v(&s));
}