use crate::rayt::*;

pub struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    pub fn new() -> Self {
        let mut world = ShapeList::new();
        let sphere = Sphere::new(Float3::new(0.0, 0.0, -1.0), 0.5);
        world.push(Box::new(sphere));

        // let sphere = Sphere::new(Float3::new(-0.1, 0.2, -1.0), 0.4);
        // world.push(Box::new(sphere));

        // let sphere = Sphere::new(Float3::new(0.5, 0.1, -1.0), 0.3);
        // world.push(Box::new(sphere));

        world.push(Box::new(Sphere::new(Float3::new(0.0, -100.5, -1.0), 100.0)));

        Self { world }
    }

    fn background_color(&self, d: Float3) -> Color {
        let t = 0.5 * (d.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl Scene for SimpleScene {
    fn camera(&self) -> Camera {
        Camera::new(
            Float3::new(4.0, 0.0, 0.0),
            Float3::new(0.0, 2.0, 0.0),
            Float3::new(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: Ray) -> Color {
        if let Some(hit) = self.world.hit(&ray, 0.001, f64::MAX){
            let target = hit.p + hit.n + Vec3::random_in_unit_sphere();
            self.trace(Ray::new(hit.p, target - hit.p)) * 0.5
        }
        else {
            self.background_color(ray.direction)
        }
    }
}

struct HitInfo {
    t: f64,
    p: Float3,
    n: Float3,
}

impl HitInfo {
    const fn new(t: f64, p: Float3, n: Float3) -> Self {
        HitInfo { t, p, n }
    }
}

trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

struct Sphere {
    center: Float3,
    radius: f64,
}

impl Sphere {
    const fn new(center: Float3, radius: f64) -> Self {
        Self { center, radius }
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
                let hit = HitInfo::new(t, p, n);
                return Some(hit);
            }

            let t = (-b + d_sqrt) / (2.0 * a);
            if t0 < t && t < t1 {
                let p = ray.at(t);
                let n = (p - self.center) / self.radius;
                let hit = HitInfo::new(t, p, n);
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
