
use crate::vec3::Vec3;
pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r_normalized  = (r * 255.999) as u32;
    let g_normalized  = (g * 255.999) as u32;
    let b_normalized  = (b * 255.999) as u32;
    println!("{}, {}, {}", r_normalized, g_normalized, b_normalized);
}