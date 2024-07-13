use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    // Method to return a darker version of the color as an example of self-modification
    pub fn darker(&mut self, amount: u8) {
        self.red = self.red.saturating_sub(amount);
        self.green = self.green.saturating_sub(amount);
        self.blue = self.blue.saturating_sub(amount);
    }

    pub fn lighter(&mut self, amount: u8) {
        self.red = self.red.saturating_add(amount);
        self.green = self.green.saturating_add(amount);
        self.blue = self.blue.saturating_add(amount);
    }
}

impl Default for Color {
    fn default() -> Self {
        Color { red: 0, green: 0, blue: 0 }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.red, self.green, self.blue)
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32
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
    color: Color,
    vertices: Vec<Vertex>
}

impl Shape {
    pub fn new() -> Self {
        Shape { color: Default::default(), vertices: vec![] }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.vertices.push(v);
    }

    pub fn color(&mut self) -> &mut Color { &mut self.color }

    pub fn vertices(&self) -> impl Iterator<Item = &Vertex> { self.vertices.iter() }

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

    pub fn is_point_in_inside(&self, point: &Vertex) -> bool {
        Shape::is_point_in_polygon_test(&mut self.vertices.iter(), point)
    }

    pub fn is_point_in_polygon_test<'a, I>(vertices: I, point: &Vertex) -> bool
    where I: Iterator<Item = &'a Vertex>, {

        let vert_vec: Vec<&Vertex> = vertices.collect();

        let mut intersections = 0;
        let num_vertices = vert_vec.len();
        let mut j = num_vertices - 1; // Previous vertex index

        for i in 0..num_vertices {
            let vi = &vert_vec[i];
            let vj = &vert_vec[j];

            if (vi.y > point.y) != (vj.y > point.y) &&
                (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x) {
                intersections += 1;
            }
            j = i;
        }

        intersections % 2 != 0
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
