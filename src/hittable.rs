
use std::rc::Rc;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, p: &Point3, outward_normal: &Vec3) -> Self {
        let front_face = ray.dir().dot(*outward_normal) < 0.0;
        Self {
            t,
            p: *p,
            normal: if front_face { *outward_normal } else { - *outward_normal},
            front_face,
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
                record = Some(r);
                closest_so_far = r.t;
            }
        }

        record
    }
}