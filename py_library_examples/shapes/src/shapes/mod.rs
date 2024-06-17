use std::fmt;

pub struct Vertex {
    x: f32,
    y: f32
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Vertex { x, y }
    }
}

// Implement Display for Vertex
impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Shape {
    vertices: Vec<Vertex>
}

impl Shape {
    pub fn new() -> Self {
        Shape { vertices: vec![] }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.vertices.push(v);
    }

    pub fn center(&self) -> Vertex {
        let count = self.vertices.len() as f32;
        let sum = self.vertices.iter().fold(Vertex::new(0.0, 0.0), |acc, v| Vertex {
            x: acc.x + v.x,
            y: acc.y + v.y,
        });

        Vertex {
            x: sum.x / count,
            y: sum.y / count,
        }
    }
}

// Implement Display for Shape
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shape with vertices:\n")?;
        for (i, vertex) in self.vertices.iter().enumerate() {
            write!(f, "  Vertex {}: {}\n", i, vertex)?;
        }
        Ok(())
    }
}
