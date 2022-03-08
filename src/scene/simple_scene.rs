use crate::rayt::*;
use crate::scene::*;
use na::vector;
use nalgebra as na;
pub struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    pub fn new() -> Self {
        let mut world = ShapeList::new();
        // world.push(
        //     ShapeBuilder::new()
        //         .image_texture("resources/shivaduke.jpg", (5.0, 5.0))
        //         .lambertian()
        //         .sphere(Float3::zero(), 1.0)
        //         .transform(Some(Float3::new(1.0, 1.0, 1.0)), Some(Quat::from_rot_x(0.25 * PI)),Some(Float3::new(0.5, 2.0, 1.0)))
        //         .build(),
        // );
        world.push(
            ShapeBuilder::new()
                .image_texture("resources/shivaduke.jpg", (1.0, 1.0))
                .diffuse_light(2.0)
                .cube()
                .transform(
                    Some(vector![1.0, 1.0, 1.0]),
                    Some(Quat::from_axis_angle(&Float3::x_axis(), -0.25 * PI)),
                    Some(vector![0.5, 1.5, 0.5]),
                )
                .build(),
        );

        world.push(Box::new(Sphere::new(
            vector![0.0, -100.5, -1.0],
            100.0,
            Arc::new(Lambertian {
                albedo: Box::new(CheckerTexture::new(
                    Box::new(ColorTexture::new(vector![0.8, 0.8, 0.8])),
                    Box::new(ColorTexture::new(vector![0.1, 0.1, 0.1])),
                    2.0,
                )),
            }),
        )));

        Self { world }
    }

    fn background_color(&self, d: Float3) -> Float3 {
        let t = 0.5 * (d.normalize().y + 1.0);
        vector![1.0, 1.0, 1.0].lerp(&vector![0.5, 0.7, 1.0], t) * 0.0
    }
}

impl SceneWithDepth for SimpleScene {
    fn camera(&self) -> Camera {
        Camera::from_lookat(
            vector![7.0, 2.0, 3.0],
            Float3::zeros(),
            Float3::y_axis(),
            20.0,
            self.aspect(),
        )
    }

    fn trace(&self, ray: Ray, depth: usize) -> Float3 {
        if let Some(hit) = self.world.hit(&ray, 0.001, f64::MAX) {
            let emitted = hit.m.emited(&ray, &hit);
            let scatter_result = if depth > 0 {
                hit.m.scatter(&ray, &hit)
            } else {
                None
            };
            if let Some(scatter_info) = scatter_result {
                emitted
                    + self
                        .trace(scatter_info.ray, depth - 1)
                        .component_mul(&scatter_info.albedo)
            } else {
                emitted
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
    fn value(&self, u: f64, v: f64, p: Float3) -> Float3;
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
    fn emited(&self, _ray: &Ray, _hit: &HitInfo) -> Float3 {
        Float3::zeros()
    }
}

pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Float3,
}

impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Float3) -> Self {
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
            if rand > schilick(self.ri, cosine) {
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

fn schilick(ri: f64, cosine: f64) -> f64 {
    let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn schlick_lerp(f0: Float3, f90: Float3, cosine: f64) -> Float3 {
    f0 + (f90 - f0) * (1.0 - cosine).powi(5)
}

pub struct ColorTexture {
    color: Float3,
}

impl ColorTexture {
    pub const fn new(color: Float3) -> Self {
        Self { color }
    }
}

impl Texture for ColorTexture {
    fn value(&self, _: f64, _: f64, _: Float3) -> Float3 {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    freq: f64,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, freq: f64) -> Self {
        Self { odd, even, freq }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Float3) -> Float3 {
        let sines = p.iter().fold(1.0, |acc, x| acc * (x * self.freq).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    pixels: Vec<Float3>,
    width: usize,
    height: usize,
    scale: (f64, f64),
}

impl ImageTexture {
    pub fn new(path: &str, scale: (f64, f64)) -> Self {
        let rgbimg = image::open(path).unwrap().to_rgb8();
        let (w, h) = rgbimg.dimensions();
        let mut image = vec![Float3::zeros(); (w * h) as usize];
        for (i, (_, _, pixel)) in image.iter_mut().zip(rgbimg.enumerate_pixels()) {
            *i = float3::from_rgb(pixel[0], pixel[1], pixel[2]);
        }
        Self {
            pixels: image,
            width: w as usize,
            height: h as usize,
            scale: scale,
        }
    }

    fn sample(&self, u: i64, v: i64) -> Float3 {
        let tu = (u as usize).clamp(0, self.width - 1);
        let tv = (v as usize).clamp(0, self.height - 1);
        self.pixels[tu + self.width * tv]
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Float3) -> Float3 {
        let u = (u * self.scale.0).fract();
        let v = (v * self.scale.1).fract();
        let x = (u * self.width as f64) as i64;
        let y = (v * self.height as f64) as i64;
        self.sample(x, y)
    }
}
