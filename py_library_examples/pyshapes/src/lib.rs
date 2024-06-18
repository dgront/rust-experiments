mod shapes;

use pyo3::{pymodule, PyResult, Python};
use pyo3::prelude::PyModule;
pub use shapes::*;

#[pymodule]
#[pyo3(name = "shapes")]
fn pyshapes(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vertex>()?;
    m.add_class::<Shape>()?;
    Ok(())
}