use std::fmt;
// use std::sync::{Arc, Mutex};
use pyo3::{pyclass, pymethods, pymodule, PyResult, Python};
use pyo3::prelude::PyModule;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Vertex {
    #[pyo3(get, set)]
    x: f32,
    #[pyo3(get, set)]
    y: f32,
}


#[pymethods]
impl Vertex {
    #[new]
    pub fn new(x: f32, y: f32) -> Self {
        Vertex { x, y }
    }

    pub fn __repr__(&self) -> String {
        format!("Vertex({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[pyclass]
pub struct Shape {
    vertices: Vec<Vertex>,
}

#[pymethods]
impl Shape {
    #[new]
    pub fn new() -> Self {
        Shape { vertices: vec![] }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
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

    pub fn is_point_in_polygon(&self, point: &Vertex) -> bool {
        let mut intersections = 0;
        let num_vertices = self.vertices.len();
        let mut j = num_vertices - 1; // Previous vertex index

        for i in 0..num_vertices {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];

            if (vi.y > point.y) != (vj.y > point.y) &&
                (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x) {
                intersections += 1;
            }
            j = i;
        }

        intersections % 2 != 0
    }

    pub fn __repr__(&self) -> String {
        let vertices: Vec<String> = self.vertices.iter().map(|v| v.__repr__()).collect();
        format!("Shape with vertices: [{}]", vertices.join(", "))
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shape with vertices:\n")?;
        for (i, vertex) in self.vertices.iter().enumerate() {
            write!(f, "  Vertex {}: {}\n", i, vertex)?;
        }
        Ok(())
    }
}

