use crate::rayt::*;

#[derive(Copy, Clone)]
pub struct Quat(Vec3, f64);

impl Quat {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Quat(Vec3::new(x, y, z), w)
    }

    pub fn from_rot(v: Vec3, rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat(v * s, c)
    }

    pub fn from_rot_x(rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat::new(s, 0.0, 0.0, c)
    }

    pub fn from_rot_y(rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat::new(0.0, s, 0.0, c)
    }
    pub fn from_rot_z(rad: f64) -> Self {
        let (s, c) = (rad * 0.5).sin_cos();
        Quat::new(0.0, 0.0, s, c)
    }

    pub const fn unit() -> Self {
        Quat::new(0.0, 0.0, 0.0, 1.0)
    }
    pub const fn zero() -> Self {
        Quat::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Quat {
    pub fn conj(&self) -> Self {
        // Vec3(=Float3)のNeg
        Quat(-self.0, self.1)
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.0.dot(rhs.0) + self.1 * rhs.1
    }

    pub fn length_squared(&self) -> f64 {
        self.0.length_squared() + self.1 * self.1
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let recip = self.length().recip();
        Quat(self.0 * recip, self.1 * recip)
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }

    pub fn w(&self) -> f64 {
        self.1
    }

    pub fn to_array(&self) -> [f64; 4] {
        let [x, y, z] = self.0.to_array();
        [x, y, z, self.1]
    }

    pub fn rotate(&self, p: Vec3) -> Vec3 {
        let [x1, y1, z1, w1] = self.to_array();
        let [x2, y2, z2] = p.to_array();

        let x = (w1 * x2 + y1 * z2) - (z1 * y2);
        let y = (w1 * y2 + z1 * x2) - (x1 * z2);
        let z = (w1 * z2 + x1 * y2) - (y1 * x2);
        let w = (x1 * x2 + y1 * y2) - (z1 * z2);
        Vec3::new(
            ((w * x1 + x * w1) - y * z1) + z * y1,
            ((w * y1 + y * w1) - z * x1) + x * z1,
            ((w * z1 + z * w1) - x * y1) + y * x1,
        )
    }
}

impl std::ops::Mul for Quat{
    type Output = Self;
    fn mul(self, rhs:Self) -> Self{
        let [x1, y1, z1, w1] = self.to_array();
        let [x2, y2, z2, w2] = rhs.to_array();
        Quat::new(
            w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            w1 * y2 + y1 * w2 + z1 * x2 - x1 * z2,
            w1 * z2 + z1 * w2 + x1 * y2 - y1 * x2,
            w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
        )
    }
}
