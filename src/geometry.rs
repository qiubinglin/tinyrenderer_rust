use std::ops::{Add, Sub, Mul, Div, Index};
use std::f64::consts;

#[derive(Copy, Clone)]
pub struct Vec3D {
    arr: [f64; 3],
}

impl Vec3D {
    #[allow(dead_code)]
    fn norm2(self) -> f64 {
        self*self
    }

    #[allow(dead_code)]
    fn norm(self) -> f64 {
        f64::sqrt(self.norm2())
    }
}

impl Index<usize> for Vec3D {
    type Output = f64;

    fn index(&self, rhs: usize) -> &f64 {
        &self.arr[rhs]
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Vec3D {
        Vec3D { arr: [ self.arr[0]+rhs.arr[0], self.arr[1]+rhs.arr[1], self.arr[2]+rhs.arr[2] ] }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Vec3D) -> Vec3D {
        Vec3D { arr: [ self.arr[0]-rhs.arr[0], self.arr[1]-rhs.arr[1], self.arr[2]-rhs.arr[2] ] }
    }
}

impl Mul<Vec3D> for Vec3D {
    type Output = f64;

    fn mul(self, rhs: Vec3D) -> f64 {
        self.arr[0]*rhs.arr[0] + self.arr[1]*rhs.arr[1] + self.arr[2]*rhs.arr[2]
    }
}

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Vec3D {
        Vec3D { arr: [self.arr[0]*rhs, self.arr[1]*rhs, self.arr[2]*rhs] }
    }
}

impl Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Vec3D {
        Vec3D { arr: [self*rhs.arr[0], self*rhs.arr[1], self*rhs.arr[2]] }
    }
}

impl Div<f64> for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: f64) -> Vec3D {
        Vec3D { arr: [self.arr[0]/rhs, self.arr[1]/rhs, self.arr[2]/rhs] }
    }
}