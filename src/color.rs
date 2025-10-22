use std::fs::File;
use std::io::Write;
use std::ops::Mul;
use crate::interval::Interval;
use crate::vec3::Vec3;
pub type Color = Vec3;

fn write_color(pixel_color: &Color) -> String {
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

    format!("{}, {}, {}\n", r_normalized, g_normalized, b_normalized)
}
pub fn write_file(image :Vec<Vec<Color>>, filename:&str) {
    let mut file = File::create(filename).expect(&format!("Unable to create file {}", filename));
    let header = format!("P3\n{} {}\n255\n", image[0].len(), image.len() );
    file.write(header.as_bytes()).expect(&format!("Unable to write to file {}", filename));
    image.iter().for_each(|row| {
        row.iter().for_each(|&color| {
            file.write(write_color(&color).as_bytes()).expect(&format!("Unable to write {}", filename));
        })
    });
}
pub fn write_file_flat(image: Vec<Color>,width:u32, height:u32, filename:&str) {
    let mut file = File::create(filename).expect(&format!("Unable to create file {}", filename));
    let header = format!("P3\n{} {}\n255\n", width, height);
    file.write(header.as_bytes()).expect(&format!("Unable to write to file {}", filename));
    image.iter().for_each(|row| {
        file.write(write_color(&row).as_bytes()).expect(&format!("Unable to write to file {}", filename));
    })
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