use std::ops::{Add, Sub, Mul, Div, Index};

pub struct Vec3D {
    arr: [f64; 3],
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
        self
    }
}