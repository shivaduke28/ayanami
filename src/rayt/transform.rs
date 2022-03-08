use crate::rayt::*;

pub struct Transform {
    shape: Box<dyn Shape>,
    position: Option<Float3>,
    rotation: Option<Quat>,
    scale: Option<Float3>,
}

impl Transform {
    pub fn new(shape: Box<dyn Shape>) -> Self {
        Self {
            shape,
            position: None,
            rotation: None,
            scale: None,
        }
    }

    pub fn translate_mut(&mut self, position: Float3) -> &Self {
        let p = self.position.unwrap_or(Float3::zeros());
        self.position = Some(p + position);
        self
    }

    pub fn rotate_mut(&mut self, rotation: Quat) -> &Self {
        let r = self.rotation.unwrap_or(Quat::identity());
        self.rotation = Some(rotation * r);
        self
    }

    pub fn scale_mut(&mut self, scale: Float3) -> &Self {
        let s = self.scale.unwrap_or(float3::one());
        self.scale = Some(s.component_mul(&scale));
        self
    }

    fn ray_object_space(&self, ray: &Ray) -> Ray {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        if let Some(pos) = self.position {
            origin -= pos;
        }
        if let Some(rot) = self.rotation {
            let rot_inv = rot.conjugate();
            origin = rot_inv * origin;
            direction = rot_inv * (direction);
        }
        if let Some(scale) = self.scale {
            origin.component_div_assign(&scale);
            direction.component_div_assign(&scale);
        }
        Ray::new(origin, direction)
    }

    fn object_to_world(&self, position: Float3) -> Float3 {
        let mut result = position;
        if let Some(scale) = self.scale {
            result = result.component_mul(&scale);
        }
        if let Some(rotation) = self.rotation {
            result = rotation * result;
        }
        if let Some(pos) = self.position {
            result += pos;
        }

        result
    }

    fn object_to_world_normal(&self, normal: Float3) -> Float3 {
        let mut result = normal;
        if let Some(scale) = self.scale {
            result = result.component_mul(&scale);
        }
        if let Some(rotation) = self.rotation {
            result = rotation * result;
        }
        result.normalize()
    }
}

impl Shape for Transform {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let ray_os = self.ray_object_space(ray);
        if let Some(hit) = self.shape.hit(&ray_os, t0, t1) {
            let p = self.object_to_world(hit.p);
            let n = self.object_to_world_normal(hit.n);
            Some(HitInfo { p: p, n: n, ..hit })
        } else {
            None
        }
    }
}
