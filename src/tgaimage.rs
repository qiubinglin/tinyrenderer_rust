use std::ops::{Index, Mul};
use std::f64;

pub struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

pub struct TGAColor {
    bgra: [u8; 4],
    bytespp: u8,
}

impl Index<usize> for TGAColor {
    type Output = u8;

    fn index(&self, rhs: usize) -> &u8 {
        &self.bgra[rhs]
    }
}

impl Mul<f64> for TGAColor {
    type Output = TGAColor;

    fn mul(self, intensity: f64) -> TGAColor {
        let clamped = {
            let intensity = f64::min(intensity, 1.0);
            f64::max(0.0, intensity)
        };
        
        let mut i = 0;
        while(i < self.bgra.len()) {
            self.bgra[i] = self.bgra[i]*clamped;
        }
        self
    }
}