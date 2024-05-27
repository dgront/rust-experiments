struct Container {
    pub elements: Vec<Rect>
}

impl Container {
    pub fn new() -> Self {
        Container{elements: vec![]}
    }
}

struct Rect {
    pub x: u16,
    pub y: u16
}


impl Rect {
    pub fn new(c: &mut Container, x: u16, y: u16) -> &mut Self {
        let o = Self{x, y};
        c.elements.push(o);
        return c.elements.last_mut().unwrap()
    }
}


fn main() {
    let mut c = Container::new();
    let r: &mut Rect = Rect::new(&mut c, 10, 10);
    r.x = 25;
    r.y = 100;
}