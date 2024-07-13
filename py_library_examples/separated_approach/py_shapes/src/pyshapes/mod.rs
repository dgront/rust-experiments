use pyo3::{Py, pyclass, pymethods, Python};
use pyo3::prelude::*;

use shapes::{Vertex, Color, Shape};

#[pyclass(name="Vertex")]
#[derive(Debug, Clone)]
pub struct PyVertex(Vertex);


#[pymethods]
impl PyVertex {
    #[new]
    pub fn new(x: f32, y: f32) -> Self {
        PyVertex{ 0: Vertex{ x, y }}
    }

    pub fn __repr__(&self) -> String { format!("{}", &self.0) }

    #[getter]
    fn get_x(&self) -> f32 { self.0.x }

    #[setter]
    fn set_x(&mut self, x: f32) { self.0.x = x; }

    #[getter]
    fn get_y(&self) -> f32 { self.0.y }

    #[setter]
    fn set_y(&mut self, y: f32) { self.0.y = y; }
}

#[pyclass(name="Color")]
#[derive(Debug, Clone, Default)]
pub struct PyColor (Color);


#[pymethods]
impl PyColor {
    #[new]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        PyColor { 0:Color::new(red, green, blue) }
    }

    pub fn darker(&mut self, amount: u8) { self.0.darker(amount) }

    pub fn lighter(&mut self, amount: u8) { self.0.lighter(amount) }

    pub fn __repr__(&self) -> String { format!("{}", &self.0) }
}


#[pyclass(name="Shape")]
pub struct PyShape {
    color: Py<PyColor>,
    vertices: Vec<PyVertex>
}


#[pymethods]
impl PyShape {
    #[new]
    pub fn new(py: Python) -> Self {
        let color = Py::new(py, PyColor::default()).unwrap();
        PyShape { color, vertices: vec![]}
    }

    #[getter]
    fn get_color(&self, py: Python) -> Py<PyColor> {
        self.color.clone_ref(py)
    }

    pub fn add_vertex(&mut self, v: PyVertex) {
        self.vertices.push(v);
    }

    pub fn is_point_in_inside(&self, point: &PyVertex) -> bool {
        Shape::is_point_in_polygon_test(self.vertices.iter().map(|v| &v.0), &point.0)
    }

    pub fn vertices(slf: PyRef<'_, Self>) -> PyResult<Py<ShapeIterator>> {
        let iter = ShapeIterator {
            iter: slf.vertices.clone().into_iter(),
        };
        Py::new(slf.py(), iter)
    }
}

#[pyclass]
pub struct ShapeIterator {
    iter: std::vec::IntoIter<PyVertex>,
}

#[pymethods]
impl ShapeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyVertex> {
        slf.iter.next()
    }
}





