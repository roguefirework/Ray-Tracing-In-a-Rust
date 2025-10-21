use std::rc::Rc;
use crate::interval::Interval;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects : Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects : Vec::new()
        }
    }
    pub fn objects(&self) -> &[Box<dyn Hittable>] {
        &self.objects
    }
    pub fn add(&mut self, object : Box<dyn Hittable>)
    {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {

        self.objects.clear()
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval : &Interval) -> Option<HitRecord> {
        let mut hit : Option<HitRecord> = None;
        let mut closest_so_far = interval.max;
        for object in self.objects.iter() {
            let maybe_hit = object.hit(ray, &Interval::new(interval.min, closest_so_far));
            if (maybe_hit.is_some()) {
                hit = maybe_hit;
                closest_so_far = hit.unwrap().t();
            }
        }
        return hit;
    }
}