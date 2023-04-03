use std::ops::{Mul,Add};

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

impl Add for Complex {
    type Output = Self;

    fn add(self, another: Self) -> Self { Self{
        real : self.real + another.real,
        img  : self.img + another.img ,}
    }

}

pub fn from_these(arg: Vec<f64>) -> Complex {
    match arg.len() {
        1 => Complex{real:arg[0], img:0.0},
        2 => Complex{real:arg[0], img:arg[1]},
        _ => panic!("Only takes vector of size 1 or 2")
    }
}

impl Complex {
    pub fn modulus_squared(&self) -> f64 {
        self.real*self.real + self.img*self.img
    }

    pub fn conjugate(&self) -> Self {
        Self{real: self.real, img: -1.0 * self.img}
    }
}

