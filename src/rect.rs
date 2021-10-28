use std::rc::Rc;
use super::vec3::{Point3, Vec3};
use super::material::Material;
use super::hittable::{Hittable, HitRecord};
use super::ray::Ray;
use super::bbox::AABB;

pub enum AARect {
    XYRect {
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: Rc<dyn Material>,
    },
    XZRect {
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Rc<dyn Material>,
    },
    YZRect {
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Rc<dyn Material>,
    }
}

impl AARect {
    pub fn new_xy(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Rc<dyn Material>) -> Self {
        AARect::XYRect{x0, x1, y0, y1, k, material}
    }

    pub fn new_xz(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Rc<dyn Material>) -> Self {
        AARect::XZRect{x0, x1, z0, z1, k, material}
    }

    pub fn new_yz(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Rc<dyn Material>) -> Self {
        AARect::YZRect{y0, y1, z0, z1, k, material}
    }
}

impl Hittable for AARect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (d0, d1, d2) = match self {
            AARect::XYRect{..} => (0, 1, 2),
            AARect::XZRect{..} => (0, 2, 1),
            AARect::YZRect{..} => (1, 2, 0)
        };

        let (min0, max0, min1, max1, k, material) = match self {
            AARect::XYRect{x0, x1, y0, y1, k, material} => (x0, x1, y0, y1, k, material),
            AARect::XZRect{x0, x1, z0, z1, k, material} => (x0, x1, z0, z1, k, material),
            AARect::YZRect{y0, y1, z0, z1, k, material} => (y0, y1, z0, z1, k, material),
        };

        let origin = ray.origin();
        let dir = ray.dir();

        let t = (k - origin[d2]) / dir[d2];
        if t < t_min || t > t_max {
            return None;
        }

        let v0 = origin[d0] + dir[d0] * t;
        let v1 = origin[d1] + dir[d1] * t;
        if v0 < *min0 || v0 > *max0 || v1 < *min1 || v1 > *max1 {
            return None;
        }

        let p = ray.at(t);
        let mut n = Vec3::default();
        n[d2] = 1.0;
        let u = (v0 - min0) / (max0 - min0);
        let v = (v1 - min1) / (max1 - min1);
        Some(HitRecord::new(ray, t, &p, &n, material.clone(), u, v))
    }
    fn bounding_box(&self) -> Option<AABB> {
        let (x0, y0, z0, x1, y1, z1) = match self {
            AARect::XYRect{x0, x1, y0, y1, k, ..} => (*x0, *y0, k - 0.0001, *x1, *y1, k + 0.0001),
            AARect::XZRect{x0, x1, z0, z1, k, ..} => (*x0, k - 0.0001, *z0, *x1, k + 0.0001, *z1),
            AARect::YZRect{y0, y1, z0, z1, k, ..} => (k - 0.0001, *y0, *z0, k + 0.0001, *y1, *z1)
        };

        Some(AABB::new(&Point3::new(x0, y0, z0), 
                        &Point3::new(x1, y1, z1)))
    }
}