
#[derive(Debug)]
pub enum ElementType {
    Hydrogen = 1,
    Helium = 2,
}


pub fn main() {
    let e = ElementType::Hydrogen;
    let ee = &e;
    println!("{:?}",ee);
}
