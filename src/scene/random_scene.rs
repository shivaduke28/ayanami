use crate::rayt::*;
use crate::scene::*;

pub struct RandomScene {
    world: ShapeList,
}

impl RandomScene {
    pub fn new() -> Self {
        let mut world = ShapeList::new();

        world.push(
            ShapeBuilder::new()
                .lambertian(Color::new(0.5, 0.5, 0.5))
                .sphere(Float3::new(0.0, -1000.0, 0.0), 1000.0)
                .build(),
        );

        for au in -11..11 {
            let a = au as f64;
            for bu in -11..11 {
                let b = bu as f64;
                let [rx, rz, material_choice] = Float3::random().to_array();
                let center = Float3::new(a + 0.9 * rx, 0.2, b + 0.9 * rz);

                world.push({
                    if material_choice < 0.8 {
                        let albedo = Color::random() * Color::random();
                        ShapeBuilder::new()
                            .lambertian(albedo)
                            .sphere(center, 0.2)
                            .build()
                    } else if material_choice < 0.95 {
                        let albedo = Color::random_limit(0.5, 1.0);
                        let fuzz = rand::random::<f64>();
                        ShapeBuilder::new()
                            .metal(albedo, fuzz)
                            .sphere(center, 0.2)
                            .build()
                    } else {
                        ShapeBuilder::new()
                            .dielectric(1.5)
                            .sphere(center, 0.2)
                            .build()
                    }
                });
            }
        }

        world.push(
            ShapeBuilder::new()
                .dielectric(1.5)
                .sphere(Float3::new(0.0, 1.0, 0.0), 1.0)
                .build(),
        );
        world.push(
            ShapeBuilder::new()
                .lambertian(Float3::new(0.4, 0.2, 0.1))
                .sphere(Float3::new(-4.0, 1.0, 0.0), 1.0)
                .build(),
        );
        world.push(
            ShapeBuilder::new()
                .metal(Color::new(0.7, 0.6, 0.5), 0.0)
                .sphere(Float3::new(4.0, 1.0, 0.0), 1.0)
                .build(),
        );

        Self { world }
    }

    fn background(&self, d: Float3) -> Color {
        let t = 0.5 * (d.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl SceneWithDepth for RandomScene {
    fn camera(&self) -> Camera {
        Camera::from_lookat(
            Float3::new(13.0, 2.0, 3.0),
            Float3::new(0.0, 0.0, 0.0),
            Float3::yaxis(),
            20.0,
            self.aspect(),
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
            self.background(ray.direction)
        }
    }
}
