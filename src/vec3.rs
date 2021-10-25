use std::{ops, fmt};
use rand::{self,Rng};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    pub v: [f64;3],
}

pub type Color = Vec3;
#[allow(dead_code)]
pub type Point3 = Vec3;

#[allow(dead_code)]
impl Vec3 {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {v: [x, y, z]}
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self{
            v: [rng.gen(), rng.gen(), rng.gen()]
        }
    }

    pub fn random_in(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self{
            v: [rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max)]
        }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.v[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.v[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.v[2]
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.v[1] * other.v[2] - self.v[2] * other.v[1],
            self.v[2] * other.v[0] - self.v[0] * other.v[2],
            self.v[0] * other.v[1] - self.v[1] * other.v[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.v[0].abs() < S && self.v[1].abs() < S && self.v[2].abs() < S
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.v[0] += other.v[0];
        self.v[1] += other.v[1];
        self.v[2] += other.v[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            v: [self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2]]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.v[0] -= other.v[0];
        self.v[1] -= other.v[1];
        self.v[2] -= other.v[2];
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            v: [self.v[0] * other.v[0],
                self.v[1] * other.v[1],
                self.v[2] * other.v[2]]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            v: [self.v[0] * other,
                self.v[1] * other,
                self.v[2] * other]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(& mut self, other: f64) {
        self.v[0] *= other;
        self.v[1] *= other;
        self.v[2] *= other;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        self * (1.0 / other)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(& mut self, other: f64) {
        *self *= 1.0 / other;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            v: [-self.v[0], -self.v[1], -self.v[2]]
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        &self.v[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);
        &mut self.v[index]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.v[0], self.v[1], self.v[2])
    }
}