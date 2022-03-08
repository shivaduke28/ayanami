use crate::rayt::*;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Float3) -> Float3;
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
            scale,
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
