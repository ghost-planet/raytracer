use super::vec3::{Point3, Vec3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let viewport_height: f64 = 2.0;
        let focol_length: f64 = 1.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let origin = Point3::default();
        let horizontal = Point3::new(viewport_width, 0.0, 0.0);
        let vertical = Point3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Point3::new(0.0, 0.0, focol_length);

        Self {
            origin, lower_left_corner,
            horizontal, vertical,
        }
    }

    pub fn gen_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin, &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin))
    }
}