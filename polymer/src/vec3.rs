#[derive(Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z}
    }
    pub fn from_float(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value
        }
    }
}

fn main() {
    let v1 = Vec3::from_float(0.0);
    println!("{:.2} {:.2}", v1.x, v1.y );
}