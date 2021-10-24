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

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: if fuzz < 1.0 {fuzz} else {1.0},
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_direction = reflect(&ray.dir(), &rec.normal);
        let scatter = reflected_direction + random_in_unit_sphere() * self.fuzz;
        if scatter.dot(rec.normal) > 0.0 {
            Some((self.albedo, Ray::new(&rec.p, &scatter)))
        } else {
            None
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

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * (v.dot(*n) * 2.0)
}