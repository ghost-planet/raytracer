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