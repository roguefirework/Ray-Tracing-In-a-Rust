use std::rc::Rc;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<'a> {
    objects : &'a mut Vec<Rc<dyn Hittable>>
}

impl HittableList<'_> {
    pub fn new<'a>() -> HittableList<'a> {
        todo!()// figure out how to make a constructor
    }
    pub fn objects(&self) -> Vec<Rc<dyn Hittable>> {
        self.objects.clone()
    }
    pub fn add(&mut self, object : Rc<dyn Hittable>) {
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
            let maybeHit = object.hit(ray, t_min, closest_so_far);
            if (maybeHit.is_some()) {
                hit = maybeHit;
                closest_so_far = hit.unwrap().t();
            }
        }
        return hit;
    }
}