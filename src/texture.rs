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