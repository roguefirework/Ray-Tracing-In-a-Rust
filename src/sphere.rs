use crate::interval::Interval;
use crate::material::Material;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::object;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}
pub struct MovingSphere {
    center: Ray,
    radius: f64,
    material: Box<dyn Material>,
}
impl MovingSphere {
    pub fn new(center0: Point3, center1: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        MovingSphere {center:Ray::new_with_time(center0, center1-center0,0.0), radius, material}
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}
impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Self { center, radius, material }
    }
    pub fn center(&self) -> Point3 {
        self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}


impl object::Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }
        let hit_position = ray.at(root);
        Some(HitRecord::new(hit_position, (hit_position - self.center) /  self.radius, ray, root, self.material.clone_box()))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let current_center = ray.at(ray.time());
        return Sphere::new(current_center,self.radius,self.material.clone_box()).hit(ray, interval);
    }
}