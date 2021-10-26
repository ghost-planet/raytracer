use super::vec3::Point3;
use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl Default for AABB {
    fn default() -> Self {
        use std::f64::MIN;
        use std::f64::MAX;
        Self::new(&Point3::new(MAX, MAX, MAX),
            &Point3::new(MIN, MIN, MIN))
    }
}

impl AABB {
    pub fn new(min: &Point3, max: &Point3) -> Self {
        Self {
            min: *min,
            max: *max,
        }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let dir = ray.dir();
        let origin = ray.origin();
        for i in 0..3 {
            let inv_d = 1.0 / dir[i];
            let mut t0 = (self.min[i] - origin[i]) * inv_d;
            let mut t1 = (self.max[i] - origin[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min {t0} else {t_min};
            let t_max = if t1 < t_max {t1} else {t_max};
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn merge(&mut self, other: &AABB) {
        let omin = other.min();
        let omax = other.max();
        
        self.min = Point3::new(fmin(self.min.x(), omin.x()),
                                fmin(self.min.y(), omin.y()),
                                fmin(self.min.z(), omin.z()));
        self.max = Point3::new(fmax(self.max.x(), omax.x()),
                                fmax(self.max.y(), omax.y()),
                                fmax(self.max.z(), omax.z()));
    }

    pub fn valid(&self) -> bool {
        self.min.x() <= self.max.x() 
        && self.min.y() <= self.max.y()
        && self.min.z() <= self.max.z()
    }
}

fn fmin(left: f64, right: f64) -> f64 {
    if left < right {
        left
    } else {
        right
    }
}

fn fmax(left: f64, right: f64) -> f64 {
    if left < right {
        right
    } else {
        left
    }
}