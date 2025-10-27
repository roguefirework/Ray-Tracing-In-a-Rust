use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone, Copy)]
pub struct AABB {
    x : Interval,
    y : Interval,
    z : Interval,
}



impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB {x,y,z}
    }
    pub fn from_bounds(a: &Point3, b: &Point3) -> AABB {
        AABB::new(
            if a.x() < b.x() { Interval::new(a.x(),b.x()) } else {Interval::new(b.x(),a.x())},
            if a.y() < b.y() { Interval::new(a.y(),b.y()) } else {Interval::new(b.y(),a.y())},
            if a.z() < b.z() { Interval::new(a.z(),b.z()) } else {Interval::new(b.z(),a.z())}
        )
    }
    pub fn empty() -> AABB {
        AABB {x:Interval::empty(), y:Interval::empty(), z:Interval::empty()}
    }
    pub fn from_aabb(p0: &AABB, p1: &AABB) -> AABB {
        AABB::new(Interval::from(p0.x(),p1.x()), Interval::from(p0.y(),p1.y()), Interval::from(p0.z(),p1.z()))
    }
    pub(crate) fn longest_axis(&self) -> usize {
        if (self.x.size() > self.y.size()) {
            if self.x.size() > self.z.size() {
                0
            }
            else {
                2
            }
        }
        else {
            if self.y.size() > self.x.size() {
                1
            }
            else {
                2
            }
        }
    }
    pub fn axis_interval(&self, n:usize) -> &Interval {
        match n {
            1 => { &self.y }
            2 => { &self.z }
            _ => { &self.x }
        }
    }
    pub fn x(&self) -> &Interval {
        &self.x
    }
    pub fn y(&self) -> &Interval {
        &self.y
    }
    pub fn z(&self) -> &Interval {
        &self.z
    }
    pub fn hit(&self, ray: &Ray, interval: &mut Interval) -> Option<Interval> {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_direction[axis];
            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;
            if t0 < t1 {
                if t0 > interval.min {
                    interval.min = t0;
                }
                if t1 < interval.max {
                    interval.max = t1;
                }
            }
            else {
                if t1 > interval.min {
                    interval.min = t1
                }
                if t0 < interval.max {
                    interval.max = t0
                }
            }
            if (interval.max <= interval.min)
            {
                return None
            }
        }
        Some(*interval)
    }
}