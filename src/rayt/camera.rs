use crate::rayt::*;
use nalgebra as na;

#[derive(Debug)]
pub struct Camera {
    pub origin: Float3,
    pub u: Float3, // left bottom -> right bottom
    pub v: Float3, // left bottom -> left up
    pub w: Float3, // world origin -> left bottom
}

impl Camera {
    pub fn new(u: Float3, v: Float3, w: Float3) -> Self {
        Self {
            origin: Float3::zeros(),
            u,
            v,
            w,
        }
    }

    // vfov = vertical hov
    // aspect = w / h
    pub fn from_lookat(
        origin: Float3,
        lookat: Float3,
        vup: na::Unit<Float3>,
        vfov: f64,
        aspect: f64,
    ) -> Self {
        let halfh = (vfov.to_radians() * 0.5).tan();
        let halfw = aspect * halfh;
        let w = (origin - lookat).normalize(); // lookat -> camera origin
        let u = vup.cross(&w);
        let v = w.cross(&u);

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
