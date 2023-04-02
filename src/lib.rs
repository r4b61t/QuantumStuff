use pyo3::prelude::*;
use rayon::prelude::*;
mod things;
pub use crate::things::*;

#[pyfunction]
fn sum_as_string(a: f64, b: f64) -> Py<PyAny> {
    let test_complex: Complex = from_these(a, b);
    let mut test_state: State = State::new(3);
    let z = from_these(0.7071067811865475, 0.0);
    let hadamard = Gate::Single { _00: z, _01: z, _10: z, _11: from_these(-1.0, 0.0) * z };
    let x =  Gate::Single { _00: from_these(0.0, 0.0), _01: from_these(1.0, 0.0),
                            _10: from_these(1.0, 0.0), _11: from_these(0.0, 0.0) };
    let cnot_0 = Gate::Control { control_bit: 0,
                            _00: from_these(0.0, 0.0), _01: from_these(1.0, 0.0),
                            _10: from_these(1.0, 0.0), _11: from_these(0.0, 0.0) };
    let cnot_1 = Gate::Control { control_bit: 1,
                            _00: from_these(0.0, 0.0), _01: from_these(1.0, 0.0),
                            _10: from_these(1.0, 0.0), _11: from_these(0.0, 0.0) };
    test_state.apply_gate(&x, vec![0]);
    test_state.apply_gate(&cnot_0, vec![1]);
    test_state.apply_gate(&cnot_1, vec![0]);
    test_state.apply_gate(&cnot_0, vec![1]);
    return Python::with_gil(|py: Python| (test_complex.real,test_complex.img,
                            test_complex.modulus_squared(),
                            (test_complex*test_complex).modulus_squared(),
                            test_state.probabilities()
    ).to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn quantum_stuff(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
