use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mandelbrot {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    width: u32,
    height: u32,
    data: Vec<u8>
}


#[wasm_bindgen]
impl Mandelbrot {

    pub fn new() -> Mandelbrot {
        let width: u32 = 300;
        let height: u32 = 300;

	let size: usize = (width*height) as usize;
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        

        return Mandelbrot {xmin: -2.0, xmax: 1.0, ymin: -1.5, ymax: 1.5, width, height, data};
    }
    
    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn data(&self) -> *const u8 { self.data.as_ptr() }
    
    pub fn redraw(&mut self) {
    
        let xstep = (self.xmax - self.xmin) / self.width as f64;
        let ystep = (self.ymax - self.ymin) / self.height as f64;
        let mut im: f64 = self.ymin;
        for row in 0..self.height {
          let mut re: f64 = self.ymin;
          for col in 0..self.width {
              re += xstep;
              let idx = self.get_index(row, col);
              self.data[idx] = self.one_point(re, im, 100);
          }
          im += ystep;
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize { return (row * self.width + column) as usize; }
    
    fn one_point(&self, re: f64, im: f64, max_steps: u8) -> u8 {
        let mut n: u8 = 0;
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        while x*x + y*y <= 4.0 && n < max_steps {
            let t = x*x - y*y + re;
            y = 2.0*x*y + im;
            x = t;
            n += 1;
        };
        
        return n;
    }
}

fn main() {
    let mut m = Mandelbrot::new();
    m.redraw();
}
