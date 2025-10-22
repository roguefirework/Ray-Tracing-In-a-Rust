use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use crate::color::{write_file, Color};
use crate::interval::Interval;
use crate::object::Hittable;
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, random_double};
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center : Point3,
    pixel00_loc : Point3,
    pixel_delta_u : Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel : u32,
    pixel_samples_scale : f64,
    max_depth : u32,
    defocus_disk_u : Vec3,
    defocus_disk_v : Vec3,
    defocus_angle: f64
}



impl Camera {

    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel : u32, max_depth : u32, look_from : Point3, look_at : Point3, up : Vec3, v_fov : f64, defocus_angle : f64, focus_dist : f64) -> Self {
        let image_height =if (image_width as f64 / aspect_ratio) as i32 > 1 {
            (image_width as f64 / aspect_ratio) as i32 } else { 1 };
        let center = look_from;
        // Camera
        let theta = degrees_to_radians(v_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate basis vectors for camera
        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        // Viewport vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Pixel deltas
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left+ 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // defocus disk
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle
        }
    }
    pub fn render(self : &Self, world : &dyn Hittable, filename : &str) {
        let bar = ProgressBar::new((self.image_height * self.image_width) as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        let image : Vec<Vec<Color>> = (0..self.image_height).into_par_iter().map_with(bar.clone(), |bar_local,y| {
            let colors : Vec<Color> = (0..self.image_width).map(move |x| {
                let color : Color = (0..self.samples_per_pixel).map(|_| {
                    let r : Ray= self.get_ray(x,y);
                    self.ray_color(&r, world, self.max_depth)
                }).sum();
                bar_local.inc(1);
                return color * self.pixel_samples_scale;
            }).collect();
            return colors
        }).collect();

        write_file(image, filename);
    }
    
    fn ray_color(self: &Self, r : &Ray, world : &dyn Hittable, depth : u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0,0.0,0.0);
        }
        let hit = world.hit(r, &Interval::new(0.001, f64::INFINITY));
        if let Some(hit) = hit {
            if let Some(scatter) = hit.material().scatter(r, &hit) {
                return *scatter.attenuation() * self.ray_color(scatter.ray(), world, depth - 1)
            }
            return Color::new(0.0,0.0,0.0);
        }

        let unit_direction : Vec3 = r.unit_direction();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0- t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
    fn get_ray(self: &Self, i : i32, j : i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_center = self.pixel00_loc +
            ((i as f64 + offset.x()) * self.pixel_delta_u) + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_center - origin;

        Ray::new_with_time(origin, ray_direction,random_double())
    }
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    fn defocus_disk_sample(self : &Self) -> Vec3 {
        let point = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * point.x() + self.defocus_disk_v * point.y()
    }
}