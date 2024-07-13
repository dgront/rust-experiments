pub mod pyshapes;

use pyo3::{Bound, pymodule, PyResult};
use pyo3::prelude::{PyModule, PyModuleMethods};

use crate::pyshapes::{PyShape, PyVertex, PyColor};

#[pymodule]
#[pyo3(name = "pyshapes")]
fn build_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyVertex>()?;
    m.add_class::<PyShape>()?;
    m.add_class::<PyColor>()?;
    Ok(())
}