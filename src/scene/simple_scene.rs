use crate::rayt::*;
use crate::scene::*;

pub struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    pub fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(Box::new(Sphere::new(
            Float3::new(0.6, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Box::new(ColorTexture::new(Color::new(
                0.1, 0.2, 0.5,
            ))))),
        )));
        world.push(Box::new(Sphere::new(
            Float3::new(-0.6, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(
                Box::new(ColorTexture::new(Color::new(0.5, 0.5, 0.5))),
                0.5,
            )),
        )));
        // world.push(Box::new(Sphere::new(
        //     Float3::new(-0.6, 0.0, -1.0),
        //     -0.45,
        //     Arc::new(Dielectric::new(1.5)),
        // )));
        // world.push(Box::new(Sphere::new(
        //     Float3::new(-0.0, -0.35, -0.8),
        //     0.15,
        //     Arc::new(Metal::new(
        //         Box::new(ColorTexture::new(Color::new(0.8, 0.8, 0.8))),
        //         0.2,
        //     )),
        // )));
        world.push(Box::new(Sphere::new(
            Float3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Lambertian {
                albedo: Box::new(ColorTexture::new(Color::new(0.8, 0.8, 0.0))),
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

pub struct HitInfo {
    pub t: f64,
    pub p: Float3,
    pub n: Float3,
    pub m: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitInfo {
    pub fn new(t: f64, p: Float3, n: Float3, m: Arc<dyn Material>, u: f64, v: f64) -> Self {
        HitInfo { t, p, n, m, u, v }
    }
}

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Float3) -> Color;
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}

pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Color,
}

impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Color) -> Self {
        Self { ray, albedo }
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
        let r = hit.n + Float3::random_in_unit_sphere();
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
        let mut reflected = ray.direction.normalize().reflect(hit.n);
        reflected = reflected + Vec3::random_in_unit_sphere() * self.fuzz;
        if reflected.dot(hit.n) > 0.0 {
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
        let reflected = ray.direction.reflect(hit.n);
        let (outward_normal, eta, cosine) = {
            let dot = ray.direction.dot(hit.n);
            if ray.direction.dot(hit.n) > 0.0 {
                (-hit.n, self.ri, self.ri * dot / ray.direction.length())
            } else {
                (hit.n, self.ri.recip(), -dot / ray.direction.length())
            }
        };
        if let Some(refracted) = ray.direction.refract(outward_normal, eta) {
            let rand = rand::random::<f64>();
            if rand > schilick(self.ri, cosine) {
                return Some(ScatterInfo::new(Ray::new(hit.p, refracted), Color::one()));
            }
        }
        Some(ScatterInfo::new(Ray::new(hit.p, reflected), Color::one()))
    }
}

fn schilick(ri: f64, cosine: f64) -> f64 {
    let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn schlick_lerp(f0: Float3, f90: Float3, cosine: f64) -> Float3 {
    f0 + (f90 - f0) * (1.0 - cosine).powi(5)
}

pub struct ColorTexture {
    color: Color,
}

impl ColorTexture {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for ColorTexture {
    fn value(&self, u: f64, v: f64, p: Float3) -> Color {
        self.color
    }
}
