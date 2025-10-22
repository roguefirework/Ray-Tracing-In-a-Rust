use crate::interval::Interval;
use crate::material::Material;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::object;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}
pub struct MovingSphere {
    center: Vec3,
    offset: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}
impl MovingSphere {
    pub fn new(center: Point3, end: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        MovingSphere {center,offset: end - center, radius, material}
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
        return hit_sphere(&self.center, &self.radius, self.material.as_ref(), ray, interval);
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let current_center = self.center + self.offset * ray.time();
        return hit_sphere(&current_center, &self.radius, self.material.as_ref(), ray, interval);
    }
}
#[inline]
fn hit_sphere<'a>(center:&Point3, radius:&f64, material:&'a (dyn Material + 'a), ray: &Ray, interval: &Interval) -> Option<HitRecord<'a>> {
    let oc = *center - *ray.origin();
    let a = ray.direction().length_squared();
    let h = ray.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
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
    Some(HitRecord::new(hit_position, (hit_position - *center) / *radius, ray, root, material))
}