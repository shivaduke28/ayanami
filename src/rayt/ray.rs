use crate::rayt::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Float3,
    pub direction: Float3,
}

impl Ray {
    pub fn new(origin: Float3, direction: Float3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Float3 {
        self.origin + self.direction * t
    }
}

pub struct HitInfo {
    pub t: f64,
    pub p: Float3,
    pub n: Float3,
    pub m: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitInfo {
    pub fn new(t: f64, p: Float3, n: Float3, m: Arc<dyn Material>, u: f64, v: f64) -> Self {
        HitInfo { t, p, n, m, u, v }
    }
}
