use super::vec3::{Vec3, Point3};

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    t: f64,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3, t: f64) -> Self {
        Self {
            orig: *orig, 
            dir: *dir,
            t,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}