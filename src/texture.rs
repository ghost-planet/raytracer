use image::{self, io::Reader as ImageReader, RgbImage, DynamicImage, ImageResult};
use std::rc::Rc;
use super::vec3::{Point3, Color};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Default, Debug)]
pub struct SolidTexture {
    color: Color,
}

impl SolidTexture {
    pub fn new(color: &Color) -> Self {
        Self {color: *color}
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>, 
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self {odd, even}
    } 
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (p.x() * 10.0).sin() * (p.y() * 10.0).sin() * (p.z() * 10.0).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: Option<RgbImage>,
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        let img = ImageTexture::load(path);
        if img.is_err() {
            return Self {
                image: None,
            };
        }

        Self {
            image: Some(img.unwrap().to_rgb8())
        }
    }

    fn load(path: &str) -> ImageResult<DynamicImage> {
        ImageReader::open(path)?.decode()
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if let Some(ref img) = self.image {
            let width = img.width();
            let height = img.height();
            let u = clamp(u, 0.0, 1.0);
            let v = 1.0 - clamp(v, 0.0, 1.0);
            let x = (u * width as f64) as u32;
            let y = (v * height as f64) as u32;
            let x = if x >= width {width - 1} else {x};
            let y = if y >= height {height - 1} else {y};

            let pixel = img.get_pixel(x, y);
            const SCALE: f64 = 1.0 / 255.0;
            Color::new(pixel[0] as f64 * SCALE,
                    pixel[1] as f64 * SCALE,
                    pixel[2] as f64 * SCALE)
        } else {
            // If we have no texture data, then return solid cyan as a debugging aid.
            Color::new(0.0, 1.0, 1.0)
        }
    }
}

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}