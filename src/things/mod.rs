mod complex;
pub use crate::things::complex::*;
use float_cmp::approx_eq;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub struct State {
    pub register: Arc<Mutex<Vec<Complex>>>,
    pub qubits: u32,
}

pub struct Gate{
    pub a: Complex,
    pub b: Complex,
    pub c: Complex,
    pub d: Complex,
} 

impl Gate{
    pub fn new(a: Complex, b: Complex, c: Complex ,d: Complex) -> Self {

        let check1 = a.modulus_squared() + c.modulus_squared() ;
        let check2 = b.modulus_squared() + d.modulus_squared() ;
        let check3 = a * c.conjugate() + b * d.conjugate() ;
        let check4 = a.conjugate() * c + b.conjugate() * d;
        let ulps = 10;
        if !(approx_eq!(f64,check1,1.0,ulps = ulps) )|
           !(approx_eq!(f64,check2,1.0,ulps = ulps) )|
           !(approx_eq!(f64,check3.real ,0.0,ulps = ulps ) )|
           !(approx_eq!(f64,check3.imag  ,0.0,ulps = ulps ) )|
           !(approx_eq!(f64,check4.real ,0.0,ulps = ulps ) )|
           !(approx_eq!(f64,check4.imag  ,0.0,ulps = ulps ) )
           {
               dbg!((check1, check2,check3.real,check4.real));
               panic!{"Matrix is not unitary"} }
        
        
        Gate{a,b,c,d}       
    }
}

impl State {
    pub fn new(qubits: u32) -> Self {
        let limit = 30;
        if (qubits < 2) | (qubits > limit){panic!("Number of qubits must be between 2 and {}",limit)}
        let dimensions: usize = 2_usize.pow(qubits);
        let mut reg = vec![Complex{real: 1.0, imag: 0.0}];
        reg.append(&mut vec![Complex { real: 0.0, imag: 0.0 }; dimensions - 1]);
        Self{
            register: Arc::new(Mutex::new(reg)),
            qubits,
        }
    }

    pub fn probabilities(&self) -> Vec<f64> {
        let reg = self.register.lock().unwrap();
        reg.par_iter()
            .map(|z| z.modulus_squared())
            .collect()
    }

    pub fn apply_gate(&mut self, gate: &Gate, targets: Vec<u32>, controls: Vec<u32>){
        fn zero_bit(i:u32 , target: u32) -> u32 {
            let mask = (1 << target) -1;
            (i & mask) | (( i & !mask) <<1)
        }

        fn mul_on(va: Complex, vb: Complex, g0: Complex, g1: Complex) ->Complex{
            va*g0 + vb*g1
        }
        
        fn all_is_one(controls: &Vec<u32>, zero_bit: usize) -> bool{
            controls.par_iter().all(|&control_bit| ((1<< control_bit ) & zero_bit) > 0 )
        }

        fn apply_on(state: &mut State, gate: &Gate, target: u32, controls: &Vec<u32>) {
        (0..=2_u32.pow(state.qubits -1) -1).into_par_iter()
            .for_each(|i| {
                let a = zero_bit(i, target) as usize;
                let b = a | (1 << target) as usize;
                if all_is_one(&controls, a) {
                    let v_a = state.register.lock().unwrap()[a];
                    let v_b = state.register.lock().unwrap()[b];
                    let (g00,g01,g10,g11) = (gate.a,gate.b,gate.c,gate.d);
                    let new_va = mul_on(v_a,v_b,g00,g01);
                    let new_vb = mul_on(v_a,v_b,g10,g11);
                    state.register.lock().unwrap()[a] = new_va;
                    state.register.lock().unwrap()[b] = new_vb;
                }
            });
        }

        targets.iter()
            .for_each(|target| {
                if target > &self.qubits {panic!("Target(s) outside qubit range")}
                apply_on(self, gate, *target, &controls)
            })
    }

    pub fn measure(&self) -> u32{
        let choices : Vec<u32> = (0..2_usize.pow(self.qubits) as u32).collect();
        let dist = WeightedIndex::new(&self.probabilities()).unwrap();
        let mut rng = thread_rng();
        choices[dist.sample(&mut rng)]
    }
}
