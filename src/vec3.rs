use std::{ops};
use std::ops::Mul;
use crate::utils::{random_double, random_double_range};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }
    pub fn random_range(min: f64, max:f64) -> Vec3 {
        Vec3::new(random_double_range(min,max), random_double_range(min,max), random_double_range(min,max))
    }
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random();
            let length_squared = p.length_squared();
            if length_squared <= 1.0  && 1e-160 < length_squared {
                return p / length_squared.sqrt();
            }
        }
    }
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double_range(-1.0,1.0), random_double_range(-1.0,1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let vec = Vec3::random_unit_vector();
        if normal.dot(vec) > 0.0 {
            vec
        } else {
            -vec
        }
    }
    pub fn refract(uv : &Vec3, normal: &Vec3, etai_over_etat:f64) -> Vec3 {
        let cos_theta = f64::min((-*uv).dot(*normal), 1.0);
        let perpendicular_comp = etai_over_etat * (*uv + cos_theta* *normal);
        let parallel_comp = -(1.0 - perpendicular_comp.length_squared()).abs().sqrt() * *normal;
        perpendicular_comp + parallel_comp
    }
    pub fn reflect(self : &Self, n: &Vec3) -> Vec3 {
        *self - (2.0 * self.dot(*n) * *n)
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn normalize(&self) -> Vec3 {
        Vec3::new(self.x() / self.length(), self.y() / self.length(), self.z() / self.length())
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other : Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other : Vec3) -> Vec3 {
        Vec3::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z , self.x * other.y - self.y * other.x)
    }

}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar : i32) -> Vec3 {
        self * scalar as f64
    }
}
impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, other : Vec3) -> Vec3 {
        other * self
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
