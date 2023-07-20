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
                a,b,c,d
                )}
    }

    #[staticmethod]
    pub fn x() -> Self {
        let one = Complex{real: 1.0 , imag : 0.0};
        let zero = Complex{real: 0.0 , imag : 0.0};
        Self{gate: Gate::new(
                zero,one,one,zero
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


/// A Python module implemented in Rust.
#[pymodule]
fn cyp_quantum_circuit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QGate>()?;
    m.add_class::<Register>()?;
    Ok(())
}
