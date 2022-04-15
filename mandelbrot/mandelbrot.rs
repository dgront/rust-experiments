use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mandelbrot {
    width: u32,
    height: u32,
    data: Vec<u8>
}


#[wasm_bindgen]
impl Mandelbrot {

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn data(&self) -> *const u8 { self.data.as_ptr() }

    fn get_index(&self, row: u32, column: u32) -> usize { return (row * self.width + column) as usize; }
}