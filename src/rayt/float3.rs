use crate::rayt::*;
use rand::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float3(pub [f64; 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Float3;

// basic
impl Float3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub const fn xaxis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    pub const fn yaxis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    pub const fn zaxis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}

// color
impl Float3 {
    pub fn r(&self) -> u8 {
        (255.0 * self.x().min(1.0).max(0.0)) as u8
    }
    pub fn g(&self) -> u8 {
        (255.0 * self.y().min(1.0).max(0.0)) as u8
    }
    pub fn b(&self) -> u8 {
        (255.0 * self.z().min(1.0).max(0.0)) as u8
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f64 / 255.0, b as f64 / 255.0, g as f64 / 255.0)
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [self.r(), self.g(), self.b()]
    }

    pub fn gamma(&self, factor: f64) -> Self {
        Self::from_iter(self.iter().map(|x| x.powf(factor)))
    }

    pub fn degamma(&self, factor: f64) -> Self {
        let recip = factor.recip();
        Self::from_iter(self.iter().map(|x| x.powf(recip)))
    }

    pub fn from_hex(hex: &[u8; 6]) -> Self {
        if let Ok(hex_str) = std::str::from_utf8(hex) {
            let r = u8::from_str_radix(&hex_str[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex_str[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex_str[4..6], 16).unwrap();
            Self::from_rgb(r, g, b)
        } else {
            panic!()
        }
    }
}

impl Float3 {
    pub const fn zero() -> Self {
        Self([0.0; 3])
    }

    pub const fn one() -> Self {
        Self([1.0; 3])
    }

    pub const fn full(value: f64) -> Self {
        Self([value; 3])
    }

    pub fn sqrt(&self) -> Self {
        Self::from_iter(self.iter().map(|x| x.sqrt()))
    }

    pub fn near_zero(&self) -> bool {
        self.iter().all(|x| x.abs() < EPS)
    }

    pub fn to_array(&self) -> [f64; 3] {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.0.iter_mut()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        let l = self.0;
        let r = rhs.0;
        l[0] * r[0] + l[1] * r[1] + l[2] * r[2]
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let l = self.0;
        let r = rhs.0;
        Self([
            l[1] * r[2] - l[2] * r[1],
            l[2] * r[0] - l[0] * r[2],
            l[0] * r[1] - l[1] * r[0],
        ])
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn lerp(&self, v: Self, t: f64) -> Self {
        *self + (v - *self) * t
    }
}

// イテレータから作る
impl FromIterator<f64> for Float3 {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut initer = iter.into_iter();
        Float3([
            initer.next().unwrap(),
            initer.next().unwrap(),
            initer.next().unwrap(),
        ])
    }
}

impl std::ops::Add for Float3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let l = self.0;
        let r = rhs.0;
        Self([l[0] + r[0], l[1] + r[1], l[2] + r[2]])
    }
}

impl std::ops::AddAssign for Float3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.0[i] += rhs.0[i]
        }
    }
}

impl std::ops::Sub for Float3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::SubAssign for Float3 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl std::ops::Neg for Float3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl std::ops::Mul for Float3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let l = self.0;
        let r = rhs.0;
        Self([l[0] * r[0], l[1] * r[1], l[2] * r[2]])
    }
}

impl std::ops::Mul<f64> for Float3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl std::ops::Div<f64> for Float3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl std::ops::Div for Float3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let l = self.0;
        let r = rhs.0;
        Self([l[0] / r[0], l[1] / r[1], l[2] / r[2]])
    }
}

impl std::ops::DivAssign<f64> for Float3 {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.0[i] /= rhs
        }
    }
}

impl Float3 {
    pub fn random() -> Self {
        Self::new(random::<f64>(), random::<f64>(), random::<f64>())
    }

    pub fn random_full() -> Self {
        Self::full(random::<f64>())
    }

    pub fn random_limit(min: f64, max: f64) -> Self {
        Self::from_iter(Self::random().iter().map(|x| min + x * (max - min)))
    }

    // 球の中に入っていない場合は捨てる
    // 棄却法という
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Self::random_limit(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }
}
