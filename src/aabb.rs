use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone, PartialEq)]
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
            Interval::new(a.x().min(b.x()), a.x().max(b.x())),
            Interval::new(a.y().min(b.y()), a.y().max(b.y())),
            Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        )
    }
    pub fn from_aabb(p0: &AABB, p1: &AABB) -> AABB {
        AABB::new(Interval::from(p0.x(),p1.x()), Interval::from(p0.y(),p1.y()), Interval::from(p1.z(),p0.z()))
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
    pub fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Interval> {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();
        let mut result : Interval = Interval::new(interval.min,interval.max);
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_direction[axis];
            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;
            if t0 > t1 {
                result = Interval::new(
                    if (t0 > result.min) {t0} else {result.min},
                    if (t0 > result.max) {t0} else {result.max}
                )
            }
            else {
                result = Interval::new(
                    if (t1 > result.min) {t1} else {result.min},
                    if (t0 > result.max) {t0} else {result.max}
                )
            }
            if (result.max <= result.min)
            {
                return None
            }
        }
        return Some(result);
    }
}