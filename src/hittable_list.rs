use crate::aabb::AABB;
use crate::interval::Interval;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct HittableList {
    aabb : AABB,
    objects : Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            aabb: AABB::from_bounds(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, 0.0)),
            objects : Vec::new()
        }
    }

    pub fn add(&mut self, object : Box<dyn Hittable>)
    {
        self.aabb = AABB::from_aabb(&self.aabb, &object.bounding_box());
        self.objects.push(object);
    }

}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval : &mut Interval) -> Option<HitRecord> {
        let mut hit : Option<HitRecord> = None;
        let mut closest_so_far = interval.max;
        for object in self.objects.iter() {
            let maybe_hit = object.hit(ray, interval);
            if let Some(new_hit) = &maybe_hit {
                closest_so_far = new_hit.t();
            }
            if maybe_hit.is_some() {
                hit = maybe_hit
            }
        }
        hit
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(HittableList::new())
    }
}