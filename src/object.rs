use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub(crate) struct HitRecord {
    position: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}
impl HitRecord {
    pub fn new(position: Vec3, outward_normal : Vec3, ray : &Ray, t: f32) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        HitRecord { position, normal, t: t, front_face: front_face }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn t(&self) -> f32 {
        self.t
    }
}
pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
