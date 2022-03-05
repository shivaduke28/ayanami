use crate::rayt::*;

pub struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    pub fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(Box::new(Sphere::new(
            Float3::new(0.6, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
        )));
        world.push(Box::new(Sphere::new(
            Float3::new(-0.6, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0)),
        )));
        world.push(Box::new(Sphere::new(
            Float3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Lambertian {
                albedo: Color::new(0.8, 0.8, 0.0),
            }),
        )));

        Self { world }
    }

    fn background_color(&self, d: Float3) -> Color {
        let t = 0.5 * (d.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl SceneWithDepth for SimpleScene {
    fn camera(&self) -> Camera {
        Camera::new(
            Float3::new(4.0, 0.0, 0.0),
            Float3::new(0.0, 2.0, 0.0),
            Float3::new(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: Ray, depth: usize) -> Color {
        if let Some(hit) = self.world.hit(&ray, 0.001, f64::MAX) {
            let scatter_result = if depth > 0 {
                hit.m.scatter(&ray, &hit)
            } else {
                None
            };
            if let Some(scatter_info) = scatter_result {
                self.trace(scatter_info.ray, depth - 1) * scatter_info.albedo
            } else {
                Color::zero()
            }
        } else {
            self.background_color(ray.direction)
        }
    }
}

struct Sphere {
    center: Float3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    fn new(center: Float3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
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
                let hit = HitInfo::new(t, p, n, Arc::clone(&self.material));
                return Some(hit);
            }

            let t = (-b + d_sqrt) / (2.0 * a);
            if t0 < t && t < t1 {
                let p = ray.at(t);
                let n = (p - self.center) / self.radius;
                let hit = HitInfo::new(t, p, n, Arc::clone(&self.material));
                return Some(hit);
            }
        }
        None
    }
}

struct ShapeList {
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

pub struct HitInfo {
    pub t: f64,
    pub p: Float3,
    pub n: Float3,
    pub m: Arc<dyn Material>,
}

impl HitInfo {
    fn new(t: f64, p: Float3, n: Float3, m: Arc<dyn Material>) -> Self {
        HitInfo { t, p, n, m }
    }
}

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}

pub struct ScatterInfo {
    ray: Ray,
    albedo: Color,
}

impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Color) -> Self {
        Self { ray, albedo }
    }
}

struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let r = hit.n + Float3::random_in_unit_sphere();
        let r = Ray::new(hit.p, r);
        Some(ScatterInfo::new(r, self.albedo))
    }
}

struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = ray.direction.normalize().reflect(hit.n);
        reflected = reflected + Vec3::random_in_unit_sphere() * self.fuzz;
        if reflected.dot(hit.n) > 0.0 {
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), self.albedo))
        } else {
            None
        }
    }
}
