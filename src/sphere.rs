use std::rc::Rc;

use super::vec3::Point3;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center: *center, 
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.dir().length_squared();
        let half_b = oc.dot(ray.dir());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        
        let p = ray.at(root);
        let n = (p - self.center) / self.radius;
        Some(HitRecord::new(ray, root, &p, &n, self.material.clone()))
    }
}