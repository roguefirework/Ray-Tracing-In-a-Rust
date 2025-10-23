use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::{MovingSphere, Sphere};
use crate::utils::{random_double, random_double_range};
use crate::vec3::{Point3, Vec3};

mod vec3;
mod color;
mod ray;
mod object;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;
mod material;
mod aabb;
mod bvh;

fn main() {
    // World
    let mut world : HittableList = HittableList::new();
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, Box::new(ground_material))));
    for i in -10..10 {
        for j in -10..10 {
            let choose_material = random_double();
            let center = Point3::new(i as f64 + 0.9 * random_double(), 0.2, j as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0.0, random_double(), 0.0);
                    world.add(Box::new(MovingSphere::new(center,center2,0.2, Box::new(material))));
                } else if choose_material < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center,0.2, Box::new(material))));
                } else {
                    let material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center,0.2, Box::new(material))));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(material1))));
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(material2))));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(material3))));



    let camera : Camera = Camera::new(16.0/9.0, 1200,500,
                                      50, Point3::new(13.0,2.0,3.0),
                                      Point3::new(0.0,0.0,0.0),
                                      Vec3::new(0.0,1.0,0.0), 20.0, 0.6, 10.0);
    camera.render(&world,"out.ppm");
}
