use std::rc::Rc;

use super::vec3::{Point3, Vec3};
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::material::Material;
use super::bbox::AABB;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(center: &Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center: *center, 
            radius,
            material,
        }
    }

    pub fn uv(&self, p: &Point3) -> (f64, f64) {
        use std::f64::consts::PI;
        use std::f64::consts::FRAC_1_PI;

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi * 0.5 * FRAC_1_PI, theta * FRAC_1_PI)
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
        let (u, v) = self.uv(&p);
        Some(HitRecord::new(ray, root, &p, &n, self.material.clone(), u, v))
    }

    fn bounding_box(&self) -> Option<AABB> {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(&(self.center - r), &(self.center + r)))
    }
}

pub struct AnimatedSphere {
    center0: Point3,
    center1: Point3,
    duration: f64,
    radius: f64,
    material: Rc<dyn Material>, 
}

#[allow(dead_code)]
impl AnimatedSphere {
    pub fn new(center0: &Point3, center1: &Point3, duration: f64, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center0: *center0,
            center1: *center1,
            duration,
            radius,
            material,
        }
    }

    pub fn center(&self, t: f64) -> Point3 {
        let t = t % self.duration;
        let f = t / self.duration;
        self.center0 * (1.0 - f) + self.center1 * f
    }

    pub fn uv(&self, p: &Point3) -> (f64, f64) {
        use std::f64::consts::PI;
        use std::f64::consts::FRAC_1_PI;

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi * 0.5 * FRAC_1_PI, theta * FRAC_1_PI)
    }
}

impl Hittable for AnimatedSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let cur_center = self.center(ray.t());

        let oc = ray.origin() - cur_center;
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
        let n = (p - cur_center) / self.radius;
        let (u, v) = self.uv(&p);
        Some(HitRecord::new(ray, root, &p, &n, self.material.clone(), u, v))
    }

    fn bounding_box(&self) -> Option<AABB> {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        let mut aabb = AABB::new(&(self.center0 - r), &(self.center0 + r));
        aabb.merge(&AABB::new(&(self.center1 - r), &(self.center1 + r)));
        Some(aabb)
    }
}