use super::vec3::{Point3, Vec3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: &Point3, look_at: &Point3, up: &Vec3, 
                fov: f64, aspect_ratio: f64) -> Self {
        let fov = fov.to_radians() / 2.0;
        let viewport_height: f64 = fov.tan() * 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = (*look_from - *look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = *look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w;

        Self {
            origin, lower_left_corner,
            horizontal, vertical,
        }
    }

    pub fn gen_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin, &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin).unit_vector())
    }
}