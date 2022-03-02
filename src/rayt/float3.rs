use crate::rayt::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float3([f64; 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Float3;

impl Float3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

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
        // self.0は[f64; 3]
        Self::from_iter(self.0.iter().map(|x| x.sqrt()))
    }

    pub fn near_zero(&self) -> bool {
        self.0.iter().all(|x| x.abs() < EPS)
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
        let l = self.0;
        let r = rhs.0;
        Self([l[0] - r[0], l[1] - r[1], l[2] - r[2]])
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
    fn div(self, rhs: Float3) -> Self {
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
