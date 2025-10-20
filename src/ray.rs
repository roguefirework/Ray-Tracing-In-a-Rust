use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Ray<'a> {
    origin: &'a Vec3,
    direction: &'a Vec3
}
impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, direction: &'a Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn origin(&self) -> &Vec3 {
        self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        self.direction
    }
    pub fn unit_direction(&self) -> Vec3 {
        self.direction.normalize()
    }
    /*
    pub fn normalize(&self) -> Ray {
        Ray::new(self.origin, &self.unit_direction())
    }
    */

    pub fn at(&self, t: f32) -> Vec3 {
        *self.origin + (t * *self.direction)
    }

}
