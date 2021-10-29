use rand::{self,Rng};
use std::rc::Rc;
use super::hittable::{Hittable, HitRecord};
use super::material::Material;
use super::vec3::Vec3;
use super::ray::Ray;
use super::bbox::AABB;

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, phase_function: Rc<dyn Material>, density: f64) -> Self {
        Self {
            boundary, phase_function,
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(ray, std::f64::NEG_INFINITY, std::f64::INFINITY);
        if rec1.is_none() {
            return None;
        }

        let mut rec1 = rec1.unwrap();
        let rec2 = self.boundary.hit(ray, rec1.t + 0.0001, std::f64::INFINITY);
        if rec2.is_none() {
            return None;
        }

        let mut rec2 = rec2.unwrap();
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let distance_inside_boundary = rec2.t - rec1.t; // * ray.dir().length();
        let mut rng = rand::thread_rng();
        let random: f64 = rng.gen_range(-1.0..1.0);
        let hit_distance = self.neg_inv_density * random.log2();
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit_distance; // / ray.dir().length();
        let p = ray.at(t);
        let n = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        
        Some(HitRecord::new(ray, t, &p, &n, self.phase_function.clone(), 0.0, 0.0))
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}