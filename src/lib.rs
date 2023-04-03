use pyo3::prelude::*;
mod things;
pub use crate::things::*;
#[pyclass]
struct Register{
    state: State
}

#[pyclass]
struct QGate{
    gate: Gate
}

#[pymethods]
impl QGate{
    #[new]
    pub fn new(a: Vec<f64>, b: Vec<f64>, c: Vec<f64> ,d: Vec<f64>) -> Self {
        Self{gate: Gate::new(a, b, c, d)}
    }
}
#[pymethods]
impl Register {
    #[new]
    fn new(qubits: u32) -> Self {
        Self{state: State::new(qubits)}
    }

    fn apply_gate(&mut self, gate: &QGate, targets: Vec<u32>, controls: Vec<u32>){
        self.state.apply_gate(&gate.gate, targets, controls)
    }
    
    fn probabilities(&self) -> Vec<f64>{
        self.state.probabilities()
    }
    
    fn measure(&self) -> u32 {
        self.state.measure()
    }
    // measure
    
}

#[pyfunction]
fn main() -> Py<PyAny> {
    let mut test_state: State = State::new(3);
    let z = from_these(vec![0.7071067811865475]);
    let x = Gate::new(vec![0.0], vec![1.0], vec![1.0],vec![0.0]);
    let h = Gate{ _00: z, _01: z, _10: z, _11: from_these(vec![-0.7071067811865475]) };
    test_state.apply_gate(&h, vec![0,1],vec![]);
    test_state.apply_gate(&x, vec![2],vec![0,1]);
    return Python::with_gil(|py: Python| (
                            test_state.probabilities(),
                            (0..10).into_iter().map(|_| test_state.measure()).collect::<Vec<_>>()
    ).to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn quantum_stuff(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    m.add_class::<QGate>()?;
    m.add_class::<Register>()?;
    Ok(())
}
