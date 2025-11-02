use crate::color::Color;
use crate::vec3::Point3;

pub trait Texture : Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
    fn to_box(&self) -> Box<dyn Texture>;
}
pub struct ConstantTexture {
    color : Color,
}
impl ConstantTexture {
    pub fn new(color : Color) -> ConstantTexture {
        ConstantTexture{color}
    }
}
impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        self.color.clone()
    }
    fn to_box(&self) -> Box<dyn Texture> {
        Box::new(ConstantTexture::new(self.color.clone()))
    }
}
pub struct CheckerTexture {
    odd : Box<dyn Texture>,
    even : Box<dyn Texture>,
    scale : f64
}
impl CheckerTexture {
    pub fn new(odd : Box<dyn Texture>, even : Box<dyn Texture>, scale : f64) -> CheckerTexture {
        CheckerTexture{odd, even, scale: 1.0 / scale}
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = (p.x() * self.scale).floor() as i32;
        let y = (p.y() * self.scale).floor() as i32;
        let z = (p.z() * self.scale).floor() as i32;
        let is_even = (x + y + z) % 2 == 0;
        if (is_even) {
            return self.even.value(u,v,p)
        }
        else {
            self.odd.value(u,v,p)
        }
    }
    fn to_box(&self) -> Box<(dyn Texture)> {
        Box::new(CheckerTexture::new(self.even.to_box(), self.odd.to_box(),  1.0 / self.scale))
    }
}