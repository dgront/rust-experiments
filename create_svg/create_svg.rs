use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn circle(cx: f32, cy: f32, r:f32) -> String {
    return format!("<circle cx='{:.2}' cy='{:.2}' r='{:.2}'/>", cx, cy, r);
}

fn main() {
    println!("{}", circle(10.0, 10.0, 1.0) );
}
