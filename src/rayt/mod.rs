pub use std::f64::consts::FRAC_1_PI;
pub use std::f64::consts::PI;
pub const PI2: f64 = PI * 2.0;
pub const EPS: f64 = 1e-6;

use na::vector;
use nalgebra as na;

mod camera;
pub(crate) mod color;
pub(crate) mod float3;
pub(crate) mod math;
mod ray;
mod render;
mod window;

pub use self::camera::Camera;
pub use self::color::*;
pub use self::math::{Float3, Quat};
pub use self::ray::Ray;
pub use self::render::*;
pub use self::window::*;
pub use std::sync::Arc;
