mod complex;
pub use crate::things::complex::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct State {
    pub register: Arc<Mutex<Vec<Complex>>>,
    pub qubits: u32,
}

pub enum Gate{
    Single {
        _00: Complex,
        _01: Complex,
        _10: Complex,
        _11: Complex,},

    Control {
        control_bit: u32,
        _00: Complex,
        _01: Complex,
        _10: Complex,
        _11: Complex,},

}


impl State {
    pub fn new(qubits: u32) -> Self {
        if qubits > 8 {panic!("Number of qubits cannot be larger than 8")}
        let dimensions: usize = 2_usize.pow(qubits);
        let mut reg = vec![from_these(1.0, 0.0)];
        reg.append(&mut vec![from_these(0.0, 0.0); dimensions - 1]);
        Self{
            register: Arc::new(Mutex::new(reg)),
            qubits,
        }
    }

    pub fn probabilities(self) -> Vec<f64> {
        let reg = self.register.lock().unwrap();
        reg.par_iter()
            .map(|z| z.modulus_squared())
            .collect()
    }

    pub fn apply_gate(&mut self, gate: &Gate, targets: Vec<u32>){
        fn zero_bit(i:u32 , target: u32) -> u32 {
            let mask = (1 << target) -1;
            (i & mask) | (( i & !mask) <<1)
        }

        fn mul_on(va: Complex, vb: Complex, g0: Complex, g1: Complex) ->Complex{
            va*g0 + vb*g1
        }
        
        fn apply_on(state: &mut State, gate: &Gate, target: u32) {
        (0..state.qubits).into_par_iter()
            .for_each(|i| {
                let a = zero_bit(i, target) as usize;
                let b = a | (1 << target) as usize;
                let v_a = state.register.lock().unwrap()[a];
                let v_b = state.register.lock().unwrap()[b];
                match gate {
                    Gate::Single{_00,_01,_10,_11} => {
                        let (g00,g01,g10,g11) = (_00,_01,_10,_11);
                        let new_va = mul_on(v_a,v_b,*g00,*g01);
                        let new_vb = mul_on(v_a,v_b,*g10,*g11);
                        state.register.lock().unwrap()[a] = new_va;
                        state.register.lock().unwrap()[b] = new_vb;
                    },
                    Gate::Control{control_bit,_00,_01,_10,_11} => {
                        let (g00,g01,g10,g11) = (_00,_01,_10,_11);
                        let new_va = mul_on(v_a,v_b,*g00,*g01);
                        let new_vb = mul_on(v_a,v_b,*g10,*g11);
                        if ((1<< control_bit) & a) > 0 {
                            state.register.lock().unwrap()[a] = new_va;
                        }
                        if ((1<< control_bit) & b) > 0 {
                            state.register.lock().unwrap()[b] = new_vb;
                        }
                    }
                }
            });
        }

        targets.iter()
            .for_each(|target| {
                if target > &self.qubits {panic!("Target(s) outside qubit range")}
                apply_on(self, gate, *target)
            })
    }
}
