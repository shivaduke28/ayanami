use crate::rayt::*;

pub fn degamma(color: Float3, factor: f64) -> Float3 {
    let recip = factor.recip();
    Float3::from_iterator(color.iter().map(|x| x.powf(recip)))
}

pub fn gamma(color: Float3, factor: f64) -> Float3 {
    Float3::from_iterator(color.iter().map(|x| x.powf(factor)))
}

pub fn float3_to_rgb(color: Float3) -> [u8; 3] {
    [f64_to_u8(color.x), f64_to_u8(color.y), f64_to_u8(color.z)]
}

pub fn f64_to_u8(x: f64) -> u8 {
    (255.0 * x.clamp(0.0, 1.0)) as u8
}

pub fn white() -> Float3 {
    vector![1.0, 1.0, 1.0]
}
