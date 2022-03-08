use na::vector;
use nalgebra as na;
pub type Quat = na::UnitQuaternion<f64>;
pub type Float3 = na::Vector3<f64>;

// 引数はnormalizeされている
pub fn reflect(incident: &Float3, normal: &Float3) -> Float3 {
    incident - normal * 2.0 * incident.dot(normal)
}

// 引数はnormalizeされている
pub fn refract(incident: &Float3, normal: &Float3, eta: f64) -> Option<Float3> {
    let ndoti = incident.dot(normal);
    let k = 1.0 - eta * eta * (1.0 - ndoti * ndoti);
    if k < 0.0 {
        None
    } else {
        Some(incident * eta - normal * (eta * ndoti + k.sqrt()))
    }
}

pub fn random_limit(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

pub fn random3_limit(min: f64, max: f64) -> Float3 {
    vector![
        random_limit(min, max),
        random_limit(min, max),
        random_limit(min, max)
    ]
}

pub fn random_in_unit_sphere() -> Float3 {
    loop {
        let point = random3_limit(-1.0, 1.0);
        if point.norm_squared() < 1.0 {
            return point;
        }
    }
}

pub fn schilick(ri: f64, cosine: f64) -> f64 {
    let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn schlick_lerp(f0: Float3, f90: Float3, cosine: f64) -> Float3 {
    f0 + (f90 - f0) * (1.0 - cosine).powi(5)
}
