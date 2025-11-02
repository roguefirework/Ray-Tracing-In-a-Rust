use crate::aabb::AABB;
use crate::interval::Interval;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use std::cmp::Ordering;

pub struct  BVHNode {
    left : Box<dyn Hittable>,
    right : Box<dyn Hittable>,
    aabb : AABB
}
impl BVHNode {
    pub fn new(objects: &mut Vec<Box<dyn Hittable>>) -> BVHNode{
        Self::make(objects,0,objects.len())
    }
    fn make(objects: &mut Vec<Box<dyn Hittable>>, start : usize, end : usize) -> BVHNode {

        let mut bbox = AABB::empty();
        for i in start..(end) {
            bbox = AABB::from_aabb(&bbox, objects[i].bounding_box());
        }
        let axis = bbox.longest_axis();

        let comparator = match axis { 1 => {// doing this in a single lambda is probably better
                BVHNode::box_x_compare
            }, 2 => {
                BVHNode::box_y_compare
            }
            _ => {
                BVHNode::box_z_compare
            }
        };
        let object_span = end - start;
        let left : Box<dyn Hittable>;
        let right : Box<dyn Hittable>;
        if object_span == 1 {
            left = objects[start].clone_box();
            right = objects[start].clone_box();
        } else if object_span == 2 {
            left = objects[start].clone_box();
            right = objects[start + 1].clone_box();
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Box::new(BVHNode::make(objects,start,mid));
            right = Box::new(BVHNode::make(objects,mid,end));
        }
        let aabb = AABB::from_aabb(left.bounding_box(), right.bounding_box());
        BVHNode { left, right, aabb }
    }
    fn box_compare(a : &Box<dyn Hittable>, b : &Box<dyn Hittable>, axis : usize ) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);
        if a_axis_interval.min < b_axis_interval.min
        {
            return Ordering::Less;
        }
        else if a_axis_interval.min > b_axis_interval.min
        {
            return Ordering::Greater;
        }
        Ordering::Equal

    }
    fn box_x_compare(a : &Box<dyn Hittable>, b : &Box<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a,b, 0)
    }
    fn box_y_compare(a : &Box<dyn Hittable>, b : &Box<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a,b, 1)
    }
    fn box_z_compare(a : &Box<dyn Hittable>, b : &Box<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a,b, 2)
    }
}
impl Hittable for BVHNode {

    fn hit(&self, ray: &Ray, interval: &mut Interval) -> Option<HitRecord> {
        if self.aabb.hit(ray, interval).is_none() {
            return None
        }
        
        let min = interval.min;
        let max = interval.max;
        let left_hit : Option<HitRecord> = self.left.hit(ray, interval);

        let right_hit = self.right.hit(ray, &mut Interval::new(min, match left_hit {
            None => { max}
            Some(ref hit) => {hit.t()}
        }));
        if right_hit.is_some() {
            right_hit
        } else {
            left_hit
        }
    }
    fn bounding_box(&self) -> &AABB {
        return &self.aabb;
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        panic!("Don't clone a BVHNode");
    }
}