use std::rc::Rc;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<'a> {
    objects : Vec<&'a dyn Hittable>
}

impl<'h> HittableList<'h> {
    pub fn new() -> Self {
        HittableList {
            objects : Vec::new()
        }
    }
    pub fn objects(&self) -> &[&'h dyn Hittable] {
        &self.objects
    }
    pub fn add(&mut self, object : &'h dyn Hittable)
    {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {

        self.objects.clear()
    }
}
impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit : Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            let maybe_hit = object.hit(ray, t_min, closest_so_far);
            if (maybe_hit.is_some()) {
                hit = maybe_hit;
                closest_so_far = hit.unwrap().t();
            }
        }
        return hit;
    }
}