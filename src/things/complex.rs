use std::ops::{Mul,Add};

use pyo3::FromPyObject;

#[derive(Copy, Clone,FromPyObject)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self , another: Self) -> Self { Self {
        real : self.real * another.real - self.imag * another.imag,
        imag  : self.real * another.imag + self.imag * another.real,}
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, another: Self) -> Self { Self{
        real : self.real + another.real,
        imag  : self.imag + another.imag ,}
    }

}

pub fn from_these(arg: Vec<f64>) -> Complex {
    match arg.len() {
        1 => Complex{real:arg[0], imag:0.0},
        2 => Complex{real:arg[0], imag:arg[1]},
        _ => panic!("Only takes vector of size 1 or 2")
    }
}

impl Complex {
    pub fn modulus_squared(&self) -> f64 {
        self.real*self.real + self.imag*self.imag
    }

    pub fn conjugate(&self) -> Self {
        Self{real: self.real, imag: -1.0 * self.imag}
    }
}

