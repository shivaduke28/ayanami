use crate::rayt::*;
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
                .build_transform()
                .translate(vector![0.0, 1.0, 0.0])
                .rotate(Quat::from_axis_angle(&Float3::x_axis(), 0.25 * PI))
                .scale(vector![1.0, 0.5, 1.5])
                .build(),
        );

        world.push(Box::new(Sphere::new(
            vector![0.0, -100.5, -1.0],
            100.0,
            Arc::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ColorTexture::new(vector![0.8, 0.8, 0.8])),
                Box::new(ColorTexture::new(vector![0.1, 0.1, 0.1])),
                2.0,
            )))),
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
