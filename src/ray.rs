use crate::vec3::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64
}
impl Ray {
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin: origin, direction: direction,time: 0.0 }
    }
    #[inline]
    pub fn new_with_time(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self { origin: origin, direction: direction,time: time }
    }
    #[inline]
    pub fn origin(&self) -> &Vec3 { &self.origin
    }
    #[inline]
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    #[inline]
    pub fn unit_direction(&self) -> Vec3 {
        self.direction.normalize()
    }
    #[inline]
    pub fn time(&self) -> f64 { self.time }
    #[inline]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (t * self.direction)
    }
}
