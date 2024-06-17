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

    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
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

