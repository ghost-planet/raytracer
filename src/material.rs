use rand::{self,Rng};

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

#[allow(dead_code)]
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
            Some((self.albedo, Ray::new(&rec.p, &scatter_direction.unit_vector())))
        }   
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[allow(dead_code)]
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
        let scatter = (reflected_direction + random_in_unit_sphere() * self.fuzz).unit_vector();
        if scatter.dot(rec.normal) > 0.0 {
            Some((self.albedo, Ray::new(&rec.p, &scatter)))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

#[allow(dead_code)]
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refract_ratio = if rec.front_face {1.0 / self.index_of_refraction} else {self.index_of_refraction};
        let refraction = refract(&ray.dir(), &rec.normal, refract_ratio);

        let mut rng = rand::thread_rng();
        let scatter_direction = match refraction {
            Some((r, reflectance)) if reflectance > rng.gen() => r,
            _ => reflect(&ray.dir(), &rec.normal),
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(&rec.p, &scatter_direction)))
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

fn refract(v: &Vec3, n: &Vec3, etai_over_etat: f64) -> Option<(Vec3, f64)> {
    let cos_theta = (-*v).dot(*n);
    let cos_theta = if cos_theta > 1.0 {1.0} else {cos_theta};
    let sin_theta_2 = 1.0 - cos_theta * cos_theta;

    // Total internal reflection
    if sin_theta_2 * etai_over_etat * etai_over_etat > 1.0 {
        return None;
    }
    let r_out_perp =  (*v + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());
    Some((r_out_perp + r_out_parallel, reflectance(cos_theta, etai_over_etat)))
} 

fn reflectance(cos_theta: f64, etai_over_etat: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - etai_over_etat) / (1.0 + etai_over_etat);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
}