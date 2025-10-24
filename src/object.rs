use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub(crate) struct HitRecord<'a> {
    position: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material : &'a dyn Material
}
impl<'a> HitRecord<'a> {
    pub fn new(position: Vec3, outward_normal : Vec3, ray : &Ray, t: f64, material : &'a (dyn Material + 'a)) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        HitRecord { position, normal, t, front_face, material }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn material(&self) -> &dyn Material {
        self.material
    }
}
pub(crate) trait Hittable : Send + Sync {
    fn hit(&self, ray: &Ray, interval : &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AABB;
}
