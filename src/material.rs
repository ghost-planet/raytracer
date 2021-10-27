use rand::{self,Rng};
use std::rc::Rc;

use super::vec3::{Vec3, Color, Point3};
use super::ray::Ray;
use super::hittable::HitRecord;
use super::texture::Texture;

pub trait Material {
    // return attenuation and scattered ray
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        // Black
        Color::default()
    }
}

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

#[allow(dead_code)]
impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + random_unit_vector();
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        if scatter_direction.near_zero() {
            Some((attenuation, Ray::new(&rec.p, &rec.normal, ray.t())))
        } else {
            Some((attenuation, Ray::new(&rec.p, &scatter_direction.unit_vector(), ray.t())))
        }   
    }
}

pub struct Metal {
    albedo: Rc<dyn Texture>,
    fuzz: f64,
}

#[allow(dead_code)]
impl Metal {
    pub fn new(albedo: Rc<dyn Texture>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 {fuzz} else {1.0},
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_direction = reflect(&ray.dir(), &rec.normal);
        let scatter = (reflected_direction + random_in_unit_sphere() * self.fuzz).unit_vector();
        if scatter.dot(rec.normal) > 0.0 {
            let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
            Some((attenuation, Ray::new(&rec.p, &scatter, ray.t())))
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

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(&rec.p, &scatter_direction, ray.t())))
    }
}

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

#[allow(dead_code)]
impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        Self {
            emit
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
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