use pyo3::prelude::*;
use pyo3::types::*;
use rayon::prelude::*;
mod complex;
pub use crate::complex::*;

#[pyfunction]
fn sum_as_string(a: f64, b: f64) -> Py<PyAny> {
    let test_complex: Complex = from_these(a, b);
    return Python::with_gil(|py: Python| vec![test_complex.real,test_complex.img,
                            test_complex.modulus_squared()
                            ,(test_complex*test_complex).modulus_squared()].to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn quantum_stuff(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
