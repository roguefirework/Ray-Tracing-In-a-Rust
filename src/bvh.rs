use crate::aabb::AABB;
use crate::interval::Interval;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub enum BVHNode {
    Leaf {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
        aabb: AABB
    },
    None
}
impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        match self {
            BVHNode::Leaf{left, right, aabb} => {
                if (aabb.hit(ray, interval)).is_none() {
                    return None
                }
                let left_hit = left.hit(ray, interval);
                let right_hit = right.hit(ray, &Interval::new(interval.min,
                                                              if left_hit.is_some()
                                                              { left_hit.as_ref()?.t() } else { interval.max }));
                if (right_hit.is_some()) {
                    right_hit
                } else {
                    left_hit
                }
            },
            BVHNode::None => {
                None
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            BVHNode::Leaf{left, right, aabb} => {
                return aabb;
            }
            BVHNode::None => {
                panic!("BVHNode::bounding_box() should not be None")
            }
        }
    }
}