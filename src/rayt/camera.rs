use crate::rayt::*;
#[derive(Debug)]
pub struct Camera {
    pub origin: Point3,
    pub u: Vec3, // left bottom -> right bottom
    pub v: Vec3, // left bottom -> left up
    pub w: Vec3, // world origin -> left bottom
}

impl Camera {
    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self {
            origin: Point3::zero(),
            u,
            v,
            w,
        }
    }

    // vfov = vertical hov
    // aspect = w / h
    pub fn from_lookat(origin: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let halfh = (vfov.to_radians() * 0.5).tan();
        let halfw = aspect * halfh;
        let w = (origin - lookat).normalize(); // lookat -> camera origin
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let uw = u * halfw;
        let vh = v * halfh;

        Self {
            origin,
            u: uw * 2.0,
            v: vh * 2.0,
            w: origin - w - uw - vh, // origin - wがスクリーンの中心のワールド座標
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.w + self.u * u + self.v * v - self.origin,
        }
    }
}
