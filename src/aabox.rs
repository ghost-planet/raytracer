use std::rc::Rc;

use super::vec3::Point3;
use super::hittable::{Hittable, HitRecord, HittableList};
use super::rect::AARect;
use super::material::Material;
use super::bbox::AABB;
use super::ray::Ray;

pub struct AABox {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl AABox {
    pub fn new(min: &Point3, max: &Point3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::default();
        
        sides.add(Rc::new(AARect::new_xy(min.x(), max.x(), min.y(), max.y(), max.z(), material.clone())));
        sides.add(Rc::new(AARect::new_xy(min.x(), max.x(), min.y(), max.y(), min.z(), material.clone())));

        sides.add(Rc::new(AARect::new_xz(min.x(), max.x(), min.z(), max.z(), max.y(), material.clone())));
        sides.add(Rc::new(AARect::new_xz(min.x(), max.x(), min.z(), max.z(), min.y(), material.clone())));

        sides.add(Rc::new(AARect::new_yz(min.y(), max.y(), min.z(), max.z(), max.x(), material.clone())));
        sides.add(Rc::new(AARect::new_yz(min.y(), max.y(), min.z(), max.z(), min.x(), material.clone())));
    
        Self {
            min: *min,
            max: *max,
            sides,
        }
    }
}

impl Hittable for AABox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(&self.min, &self.max))
    }
}