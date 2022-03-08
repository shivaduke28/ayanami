use crate::rayt::*;

pub struct CornelBoxScene {
    shapes: ShapeList,
}

impl CornelBoxScene {
    pub fn new() -> Self {
        let mut shapes = ShapeList::new();
        let red = float3::new(0.64, 0.05, 0.05);
        let white = float3::fill(0.73);
        let green = float3::new(0.12, 0.45, 0.15);

        let size = 550.0;
        let half = size * 0.5;

        shapes.push(
            ShapeBuilder::new()
                .color_texture(white)
                .lambertian()
                .rect_xy(-half, half, 0.0, size, -half)
                .build(),
        );
        shapes.push(
            ShapeBuilder::new()
                .color_texture(white)
                .lambertian()
                .rect_xy(-half, half, 0.0, size, -half)
                .build_transform()
                .rotate(Quat::from_axis_angle(&Float3::y_axis(), PI))
                .build(),
        );
        shapes.push(
            ShapeBuilder::new()
                .color_texture(green)
                .lambertian()
                .rect_xy(-half, half, 0.0, size, -half)
                .build_transform()
                .rotate(Quat::from_axis_angle(&Float3::y_axis(), PI * 0.5))
                .build(),
        );
        shapes.push(
            ShapeBuilder::new()
                .color_texture(red)
                .lambertian()
                .rect_xy(-half, half, 0.0, size, -half)
                .build_transform()
                .rotate(Quat::from_axis_angle(&Float3::y_axis(), -PI * 0.5))
                .build(),
        );
        shapes.push(
            ShapeBuilder::new()
                .color_texture(white)
                .lambertian()
                .rect_xz(-half, half, -half, half, 0.0)
                .build(),
        );
        shapes.push(
            ShapeBuilder::new()
                .color_texture(white)
                .lambertian()
                .rect_xy(-half, half, -half, half, -size)
                .build_transform()
                .rotate(Quat::from_axis_angle(&Float3::x_axis(), PI * 0.5))
                .build(),
        );

        let lsize = half * 0.25;
        shapes.push(
            ShapeBuilder::new()
                .color_texture(color::white())
                .diffuse_light(16.0)
                .rect_xz(-lsize, lsize, -lsize, lsize, 0.0)
                .build_transform()
                .rotate(Quat::from_axis_angle(&Float3::x_axis(), PI))
                .translate(float3::new(0.0, size - 5.0, 0.0))
                .build(),
        );

        shapes.push(
            ShapeBuilder::new()
                .color_texture(white)
                .lambertian()
                .cube()
                .build_transform()
                .scale(float3::new(100.0, 140., 100.0))
                .translate(float3::new(80.0, 70.0, 0.0))
                .rotate(Quat::from_axis_angle(&Float3::y_axis(), PI * 0.25))
                .build(),
        );

        shapes.push(
            ShapeBuilder::new()
                .color_texture(white)
                .lambertian()
                .cube()
                .build_transform()
                .scale(float3::new(200.0, 300., 200.0))
                .translate(float3::new(-160.0, 150.0, -100.0))
                .rotate(Quat::from_axis_angle(&Float3::y_axis(), -PI * 0.1))
                .build(),
        );

        Self { shapes }
    }
}

impl SceneWithDepth for CornelBoxScene {
    fn camera(&self) -> Camera {
        Camera::from_lookat(
            float3::new(0.0, 278.0, 880.0),
            float3::new(0.0, 278.0, 0.0),
            float3::new(0.0, 1.0, 0.0),
            40.0,
            self.aspect(),
        )
    }
    fn width(&self) -> u32 {
        200
    }
    fn height(&self) -> u32 {
        200
    }

    fn trace(&self, ray: Ray, depth: usize) -> Float3 {
        if let Some(hit) = self.shapes.hit(&ray, 0.001, f64::MAX) {
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
            float3::fill(0.0)
        }
    }
}
