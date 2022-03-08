use crate::rayt::*;

pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Float3,
}

impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Float3) -> Self {
        Self { ray, albedo }
    }
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
    fn emited(&self, _ray: &Ray, _hit: &HitInfo) -> Float3 {
        Float3::zeros()
    }
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let r = hit.n + math::random_in_unit_sphere();
        let r = Ray::new(hit.p, r);
        Some(ScatterInfo::new(r, self.albedo.value(hit.u, hit.v, hit.p)))
    }
}

pub struct Metal {
    albedo: Box<dyn Texture>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Box<dyn Texture>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = math::reflect(&ray.direction.normalize(), &hit.n);
        reflected = reflected + math::random_in_unit_sphere() * self.fuzz;
        if reflected.dot(&hit.n) > 0.0 {
            Some(ScatterInfo::new(
                Ray::new(hit.p, reflected),
                self.albedo.value(hit.u, hit.v, hit.p),
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub const fn new(ri: f64) -> Self {
        Self { ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        // 反射方向
        let reflected = math::reflect(&ray.direction, &hit.n);
        let (outward_normal, eta, cosine) = {
            let dot = ray.direction.dot(&hit.n);
            if ray.direction.dot(&hit.n) > 0.0 {
                (-hit.n, self.ri, self.ri * dot / ray.direction.norm())
            } else {
                (hit.n, self.ri.recip(), -dot / ray.direction.norm())
            }
        };
        if let Some(refracted) = math::refract(&ray.direction, &outward_normal, eta) {
            let rand = rand::random::<f64>();
            if rand > math::schilick(self.ri, cosine) {
                return Some(ScatterInfo::new(Ray::new(hit.p, refracted), float3::one()));
            }
        }
        Some(ScatterInfo::new(
            Ray::new(hit.p, reflected),
            na::vector![1.0, 1.0, 1.0],
        ))
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
    intensity: f64,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>, intensity: f64) -> Self {
        Self { emit, intensity }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &HitInfo) -> Option<ScatterInfo> {
        None
    }

    fn emited(&self, _ray: &Ray, hit: &HitInfo) -> Float3 {
        self.emit.value(hit.u, hit.v, hit.p) * self.intensity
    }
}
