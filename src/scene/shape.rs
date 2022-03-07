use crate::rayt::*;
use crate::scene::*;

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

pub struct Sphere {
    center: Float3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Float3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn uv(p: Float3) -> (f64, f64) {
        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        (
            1.0 - (phi + PI) * 0.5 * FRAC_1_PI,
            (theta + PI * 0.5) * FRAC_1_PI,
        )
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let d = ray.direction;
        let a = d.length_squared();
        let b = 2.0 * d.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        if d > 0.0 {
            let d_sqrt = d.sqrt();
            let t = (-b - d_sqrt) / (2.0 * a);
            if t0 < t && t < t1 {
                let p = ray.at(t);
                let n = (p - self.center) / self.radius;
                let (u, v) = Self::uv(p);
                let hit = HitInfo::new(t, p, n, Arc::clone(&self.material), u, v);
                return Some(hit);
            }

            let t = (-b + d_sqrt) / (2.0 * a);
            if t0 < t && t < t1 {
                let p = ray.at(t);
                let n = (p - self.center) / self.radius;
                let (u, v) = Self::uv(p);
                let hit = HitInfo::new(t, p, n, Arc::clone(&self.material), u, v);
                return Some(hit);
            }
        }
        None
    }
}

pub struct ShapeList {
    pub objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> Self {
        ShapeList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = t1;

        for object in &(self.objects) {
            if let Some(info) = object.hit(ray, t0, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }

        hit_info
    }
}

pub struct ShapeBuilder {
    texture: Option<Box<dyn Texture>>,
    material: Option<Arc<dyn Material>>,
    shape: Option<Box<dyn Shape>>,
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self {
            texture: None,
            material: None,
            shape: None,
        }
    }
    pub fn lambertian(mut self, albedo: Color) -> Self {
        self.material = Some(Arc::new(Lambertian::new(Box::new(ColorTexture::new(
            albedo,
        )))));
        self
    }

    pub fn metal(mut self, albedo: Color, fuzz: f64) -> Self {
        self.material = Some(Arc::new(Metal::new(
            Box::new(ColorTexture::new(albedo)),
            fuzz,
        )));
        self
    }

    pub fn dielectric(mut self, ri: f64) -> Self {
        self.material = Some(Arc::new(Dielectric::new(ri)));
        self
    }

    pub fn sphere(mut self, center: Float3, radius: f64) -> Self {
        self.shape = Some(Box::new(Sphere::new(
            center,
            radius,
            self.material.unwrap(),
        )));
        self.material = None;
        self
    }

    pub fn build(self) -> Box<dyn Shape> {
        self.shape.unwrap()
    }

    pub fn image_texture(mut self, path: &str, scale: (f64, f64)) -> Self {
        self.texture = Some(Box::new(ImageTexture::new(path, scale)));
        self
    }
}
