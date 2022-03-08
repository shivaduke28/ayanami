use crate::rayt::*;

pub fn one() -> Float3 {
    fill(1.0)
}

pub fn fill(x: f64) -> Float3 {
    vector![x, x, x]
}

pub fn from_rgb(r: u8, g: u8, b: u8) -> Float3 {
    vector![r as f64 / 255.0, b as f64 / 255.0, g as f64 / 255.0]
}
