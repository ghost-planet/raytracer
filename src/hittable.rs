
use std::rc::Rc;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};
use super::material::Material;
use super::bbox::AABB;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, p: &Point3, outward_normal: &Vec3, material: Rc<dyn Material>) -> Self {
        let front_face = ray.dir().dot(*outward_normal) < 0.0;
        Self {
            t,
            p: *p,
            normal: if front_face { *outward_normal } else { - *outward_normal},
            front_face,
            material,
        }
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let mut ret = Self {
            objects: Vec::new()
        };
        ret.add(object);
        ret
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for o in self.objects.iter() {
            if let Some(r) = o.hit(ray, t_min, closest_so_far) {
                closest_so_far = r.t;
                record = Some(r);
            }
        }

        record
    }

    fn bounding_box(&self) -> Option<AABB> {
        let mut aabb = AABB::default();

        for o in self.objects.iter() {
            if let Some(ref r) = o.bounding_box() {
                aabb.merge(r);
            } else {
                return None;
            }
        }

        Some(aabb)
    }
}