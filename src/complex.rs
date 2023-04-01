use pyo3::prelude::*;
use rayon::prelude::*;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub img: f64,
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self , another: Self) -> Self { Self {
        real : self.real * another.real - self.img * another.img,
        img  : self.real * another.img + self.img * another.real,}
    }
}

pub fn from_these(a: f64, b:f64) -> Complex {
    Complex{real:a, img:b}
}

impl Complex {
    pub fn modulus_squared(self) -> f64 {
        self.real*self.real + self.img*self.img
    }
}

