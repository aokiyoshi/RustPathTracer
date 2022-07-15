extern crate fast_inv_sqrt;

use image::Rgb;
use rand::prelude::*;
use std::ops::{Add, AddAssign, Mul, Not, Rem, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn mul(&self, k: f64) -> Vec3 {
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
    pub fn norm(&self) -> Vec3 {
        let invl = fast_inv_sqrt::InvSqrt64::inv_sqrt64(
            self.x * self.x + self.y * self.y + self.z * self.z,
        );
        Vec3 {
            x: self.x * invl,
            y: self.y * invl,
            z: self.z * invl,
        }
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn _print(&self) {
        println!("({},{},{})", self.x, self.y, self.z)
    }
    pub fn torgb(&self) -> Rgb<u8> {
        Rgb([
            self.x.min(255.0).max(0.0) as u8,
            self.y.min(255.0).max(0.0) as u8,
            self.z.min(255.0).max(0.0) as u8,
        ])
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Rem<f64> for Vec3 {
    type Output = Self;
    fn rem(self, k: f64) -> Self {
        Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Not for Vec3 {
    type Output = Self;
    fn not(self) -> Self {
        Self {
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z,
        }
    }
}

pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn vec0() -> Vec3 {
    Vec3 {
        x: 0_f64,
        y: 0_f64,
        z: 0_f64,
    }
}

pub fn _csh(u1: f64, u2: f64) -> Vec3 {
    let r = u1.sqrt();
    let theta = 2.0 * std::f64::consts::PI * u2;
    let x = r * theta.cos();
    let y = r * theta.sin();
    new(x, y, 0_f64.max(1.0 - u1).sqrt())
}

pub fn _rng_vec(factor: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    let z: f64 = rng.gen();
    Vec3 {
        x: x * factor,
        y: y * factor,
        z: z * factor,
    }
}
