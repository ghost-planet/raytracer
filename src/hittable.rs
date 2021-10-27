
use rand::{self,Rng};
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
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, p: &Point3, outward_normal: &Vec3, material: Rc<dyn Material>, u: f64, v: f64) -> Self {
        let front_face = ray.dir().dot(*outward_normal) < 0.0;
        Self {
            t,
            p: *p,
            normal: if front_face { *outward_normal } else { - *outward_normal},
            front_face,
            material,
            u, v,
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

pub struct BVH {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: AABB,
}

impl BVH {
    pub fn new(mut objects: Vec<Rc<dyn Hittable>>) -> Self {
        assert!(objects.len() > 1);

        let mut rng = rand::thread_rng();
        BVH::build_bvh(&mut rng, &mut objects[..])
    }

    fn build_bvh<R: Rng>(rng: &mut R, objects: &mut [Rc<dyn Hittable>]) -> Self {
        let comparator = |axis| {
            move |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
                a.bounding_box().unwrap().min()[axis].partial_cmp(&b.bounding_box().unwrap().min()[axis]).unwrap()
            }
        };

        let comparators = [
            comparator(0),
            comparator(1),
            comparator(2),
        ];
        objects.sort_by(comparators[rng.gen_range(0..3)]);

        let len = objects.len();
        if len == 1 {
            return Self {
                left: objects[0].clone(),
                right: objects[0].clone(),
                bounding_box: objects[0].bounding_box().unwrap(),
            };
        } 
        
        let (left, right): (Rc<dyn Hittable>, Rc<dyn Hittable>) = 
            if len == 2 {
                (objects[0].clone(), objects[1].clone())
            } else {
                let mid = len / 2;
                let left = Rc::new(BVH::build_bvh(rng, &mut objects[0..mid]));
                let right = Rc::new(BVH::build_bvh(rng, &mut objects[mid..]));
                (left, right)
            };
        
        let mut bbox = left.bounding_box().unwrap();
        bbox.merge(&right.bounding_box().unwrap());
        Self {
            left, right,
            bounding_box: bbox,
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        if let Some(left) = self.left.hit(ray, t_min, t_max) {
            match self.right.hit(ray, t_min, left.t) {
                Some(r) => Some(r),
                None => Some(left),
            }
        } else {
            self.right.hit(ray, t_min, t_max)
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bounding_box)
    }
}