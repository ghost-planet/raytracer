use rand::{self,Rng};

use super::vec3::{Point3, Vec3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    aperture: f64,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    shutter_duration: f64,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(look_from: &Point3, look_at: &Point3, up: &Vec3, 
                fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, shutter_duration: f64) -> Self {
        let fov = fov.to_radians() / 2.0;
        let viewport_height: f64 = fov.tan() * 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = (*look_from - *look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = *look_from;
        // Now horizontal and vertical are focus plane's directional vector
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;

        Self {
            origin, lower_left_corner,
            horizontal, vertical,
            aperture,
            u, v, _w: w,
            shutter_duration,
        }
    }

    pub fn gen_ray(&self, u: f64, v: f64) -> Ray {
        let mut rng = rand::thread_rng();
        let offset = random_in_unit_disk() * (self.aperture * 0.5);
        let offset = self.u * offset.x() + self.v * offset.y();
        // (self.lower_left_corner + self.horizontal * u + self.vertical * v) is a point in the focus plane,
        // so if the ray hit a point in focus plane then it will be focus, otherwise it will be defocus
        Ray::new(&(self.origin + offset), 
                &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset).unit_vector(),
                rng.gen_range(0.0..self.shutter_duration))
    }
}

fn random_in_unit_disk() -> Point3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            break p
        }
    }
}