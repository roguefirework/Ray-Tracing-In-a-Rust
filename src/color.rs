use crate::interval::Interval;
use crate::vec3::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use std::ops::Mul;
pub type Color = Vec3;

fn map_color(pixel_color: &Color) -> Rgb<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity: Interval = Interval::new(0.0, 0.999);
    let r_normalized  = (intensity.clamp(r) * 256.0) as u32;
    let g_normalized  = (intensity.clamp(g) * 256.0) as u32;
    let b_normalized  = (intensity.clamp(b) * 256.0) as u32;

    Rgb([r_normalized as u8, g_normalized as u8, b_normalized as u8])
}
pub fn write_file(image :Vec<Vec<Color>>, filename:&str) {
    let filename = filename;
    let mut buffer: RgbImage = ImageBuffer::new(image[0].len() as u32,image.len()  as u32);
    println!("Writing image to {}", filename);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let color = image[y as usize][x as usize];
        *pixel = map_color(&color);
    }
    buffer.save(filename).unwrap();
}
pub fn linear_to_gamma(linear_component : f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Color {
        Color::new(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}