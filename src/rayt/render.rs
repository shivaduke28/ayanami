use crate::rayt::*;
use image::{Rgb, RgbImage};
use rayon::prelude::*;

use std::{fs, path::Path};

const OUTPUT_FILE_NAME: &str = "render.png";
const BACKUP_FILE_NAME: &str = "render_bak.png";
const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 200;
const SAMPLES_PER_PIXEL: usize = 10;
const GAMMA_FACTOR: f64 = 2.2;
const MAX_RAY_BOUNCE_DEPTH: usize = 50;

pub fn backup() {
    let output_path = Path::new(OUTPUT_FILE_NAME);
    if output_path.exists() {
        println!("backup {:?} -> {:?}", OUTPUT_FILE_NAME, BACKUP_FILE_NAME);
        fs::rename(OUTPUT_FILE_NAME, BACKUP_FILE_NAME).unwrap();
    }
}

pub trait Scene {
    fn camera(&self) -> Camera;
    fn trace(&self, ray: Ray) -> Float3;
    fn width(&self) -> u32 {
        IMAGE_WIDTH
    }
    fn height(&self) -> u32 {
        IMAGE_HEIGHT
    }
    fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }
    fn spp(&self) -> usize {
        SAMPLES_PER_PIXEL
    }
}

pub trait SceneWithDepth {
    fn camera(&self) -> Camera;
    fn trace(&self, ray: Ray, depth: usize) -> Float3;
    fn width(&self) -> u32 {
        IMAGE_WIDTH
    }
    fn height(&self) -> u32 {
        IMAGE_HEIGHT
    }
    fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }
    fn spp(&self) -> usize {
        SAMPLES_PER_PIXEL
    }
}

// Syncはpar_iter_mut().for_eachに必要
pub fn render(scene: impl Scene + Sync) {
    backup();

    let camera = scene.camera();
    let mut img = RgbImage::new(scene.width(), scene.height());
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let u = *x as f64 / (scene.width() - 1) as f64;
            let v = (scene.height() - *y - 1) as f64 / (scene.height() - 1) as f64;
            let ray = camera.ray(u, v);
            let rgb = color::float3_to_rgb(scene.trace(ray));
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });

    img.save(OUTPUT_FILE_NAME).unwrap();
    draw_in_window(BACKUP_FILE_NAME, img).unwrap();
}

pub fn render_aa(scene: impl SceneWithDepth + Sync) {
    backup();

    let camera = scene.camera();
    let mut img = RgbImage::new(scene.width(), scene.height());
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let mut pixel_color = (0..scene.spp())
                .into_iter()
                .fold(Float3::zeros(), |acc, _| {
                    let rx = rand::random::<f64>();
                    let ry = rand::random::<f64>();
                    let u = (*x as f64 + rx) / (scene.width() - 1) as f64;
                    let v = ((scene.height() - *y - 1) as f64 + ry) / (scene.height() - 1) as f64;
                    let ray = camera.ray(u, v);
                    acc + scene.trace(ray, MAX_RAY_BOUNCE_DEPTH)
                });

            pixel_color /= scene.spp() as f64;
            let rgb = color::float3_to_rgb(color::degamma(pixel_color, GAMMA_FACTOR));
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });

    img.save(OUTPUT_FILE_NAME).unwrap();
    draw_in_window(BACKUP_FILE_NAME, img).unwrap();
}
