use crate::vec3::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ray {
    origin: Box<Vec3>,
    direction: Box<Vec3>
}
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin: Box::new(origin), direction: Box::new(direction) }
    }
    pub fn origin(&self) -> &Vec3 {
        self.origin.as_ref()
    }
    pub fn direction(&self) -> &Vec3 {
        self.direction.as_ref()
    }
    pub fn unit_direction(&self) -> Vec3 {
        self.direction.normalize()
    }

    pub fn at(&self, t: f64) -> Vec3 {
        *self.origin + (t * *self.direction)
    }

}
