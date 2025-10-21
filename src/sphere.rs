use crate::{object};
use crate::color::Color;
use crate::interval::Interval;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f32,
}
impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
    pub fn center(&self) -> Point3 {
        self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}
impl object::Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<object::HitRecord> {
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
        if (!interval.surrounds(root)) {
            root = (h + sqrt_discriminant) / a;
            if (!interval.surrounds(root)) {
                return None;
            }
        }
        let hit_position = ray.at(root);
        return Some(HitRecord::new(hit_position, (hit_position - self.center) /  self.radius, ray, root));
    }
}