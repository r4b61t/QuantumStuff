use core::panic;

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
    pub fn new(matrix : Vec<Vec<Complex>>) -> Self {
        if matrix.len() != 2 {panic!("Only support single gate. Matrix size must be 2 by 2")}
        if (matrix[0].len() != 2) | (matrix[1].len() != 2)
            {panic!("Only support single gate. Matrix size must be 2 by 2")}
        let a = matrix[0][0];
        let b = matrix[0][1];
        let c = matrix[1][0];
        let d = matrix[1][1];
        Self{gate: Gate::new(
                vec!(a.real,a.imag), vec!(b.real,b.imag),
                vec!(c.real,c.imag), vec!(d.real,d.imag)
                )}
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
    let h = Gate{ _00: z, _01: z, _10: z, _11: from_these(vec![-0.7071067811865475]) };
    test_state.apply_gate(&h, vec![0],vec![]);
    test_state.apply_gate(&h, vec![1],vec![0]);
    return Python::with_gil(|py: Python| (
                            test_state.probabilities()
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
