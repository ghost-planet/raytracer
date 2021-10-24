use super::vec3::{Vec3, Color};
use super::ray::Ray;
use super::hittable::HitRecord;

pub trait Material {
    // return attenuation and scattered ray
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self {
            albedo: *albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            Some((self.albedo, Ray::new(&rec.p, &rec.normal)))
        } else {
            Some((self.albedo, Ray::new(&rec.p, &scatter_direction)))
        }   
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}