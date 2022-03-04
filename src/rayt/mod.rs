pub use std::f64::consts::FRAC_1_PI;
pub use std::f64::consts::PI;
pub const PI2: f64 = PI * 2.0;
pub const EPS: f64 = 1e-6;

mod float3;
mod quat;
mod ray;
mod camera;
mod window;
mod render;
pub use self::float3::{Color, Float3, Point3, Vec3};
pub use self::quat::Quat;
pub use self::ray::Ray;
pub use self::camera::Camera;
pub use self::window::*;
pub use self::render::*;