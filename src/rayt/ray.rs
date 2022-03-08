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
