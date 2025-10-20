use crate::color::{write_color, Color};
use crate::hittable_list::HittableList;
use crate::object::Hittable;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::sphere::Sphere;
mod vec3;
mod color;
mod ray;
mod object;
mod sphere;
mod hittable_list;
mod utils;
mod interval;

fn ray_color(r : &Ray, world : &dyn Hittable) -> Color {
    let hit = world.hit(r, 0.0, f32::INFINITY);
    if (hit.is_some()) {
        return 0.5 * (hit.unwrap().normal() + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction : Vec3 = r.unit_direction();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0- t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}


fn main() {
    // Image setup
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height : i32 = if (image_width as f32 / aspect_ratio) as i32 > 1 {
        (image_width as f32 / aspect_ratio) as i32 } else { 1 };

    // World
    let mut world : HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Point3::new(0.0,0.0,0.0);

    // Viewport vectors
    let viewport_u = Vec3::new(viewport_width,0.0,0.0);
    let viewport_v = Vec3::new(0.0,-viewport_height,0.0);
    // Pixel deltas
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0,0.0,focal_length) -
        viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left+ 0.5 * (pixel_delta_u + pixel_delta_v);


    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);
            let color = ray_color(&ray, &world);
            write_color(&color);
        }
    }
}
