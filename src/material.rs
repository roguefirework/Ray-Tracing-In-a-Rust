use crate::color::Color;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::Vec3;

pub struct ScatterData {
    attenuation: Box<Color>,
    ray: Box<Ray>
}
impl ScatterData {
    pub fn new(attenuation: Box<Color>, ray : Box<Ray>) -> Self {
        ScatterData {attenuation, ray}
    }
    pub fn attenuation(&self) -> &Color {
        self.attenuation.as_ref()
    }
    pub fn ray(&self) -> &Ray {
        self.ray.as_ref()
    }
}

pub trait Material : Send + Sync{
    fn scatter(self : &Self, ray_in : &Ray, hit_data : &HitRecord) -> Option<ScatterData>;
    fn clone_box (&self) -> Box<dyn Material>;
}

pub struct Lambertian {
    albedo : Color,
}
impl Lambertian {
    pub fn new(albedo : Color) -> Lambertian {
        Lambertian {albedo}
    }
}
impl Material for Lambertian {
    fn scatter(self: &Self, _ray_in: &Ray, hit_data: &HitRecord) -> Option<ScatterData> {
        let direction = hit_data.normal() + Vec3::random_unit_vector();

        let new_ray;
        if direction.near_zero()
        {
            new_ray = Ray::new_with_time(hit_data.position(), hit_data.normal(), _ray_in.time());
        }
        else {
            new_ray = Ray::new_with_time(hit_data.position(), direction, _ray_in.time());
        }

        Some(ScatterData::new(Box::new(self.albedo), Box::new(new_ray)))
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.albedo))
    }
}

pub struct Metal {
    albedo : Color,
    fuzz : f64,
}
impl Metal {
    pub fn new(albedo : Color, fuzz : f64) -> Metal {
        Metal {albedo, fuzz}
    }
}
impl Material for Metal {
    fn scatter(self: &Self, ray_in: &Ray, hit_data: &HitRecord) -> Option<ScatterData> {
        let reflected = ray_in.direction().reflect(hit_data.normal()).normalize() + (self.fuzz * Vec3::random_unit_vector());
        if reflected.dot(hit_data.normal()) <= 0.0 {
            return None;
        }
        Some(ScatterData::new(Box::new(self.albedo), Box::new(Ray::new_with_time(hit_data.position(), reflected,ray_in.time()))))
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.albedo, self.fuzz))
    }
}
pub struct Dielectric {
    refractive_index : f64,
}
impl Dielectric {
    pub fn new(refractive_index : f64) -> Dielectric {
        Dielectric {refractive_index}
    }
    fn reflectance(cosine: f64, refractive_index : f64) -> f64 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
impl Material for Dielectric {
    fn scatter(self: &Self, ray_in: &Ray, hit_data: &HitRecord) -> Option<ScatterData> {
        let ri = if hit_data.front_face() {1.0 / self.refractive_index} else {self.refractive_index};
        let unit_direction = ray_in.unit_direction();

        let cos_theta = f64::min((-unit_direction).dot(hit_data.normal()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cant_refract = ri * sin_theta > 1.0;

        if cant_refract || Dielectric::reflectance(cos_theta, ri) > random_double(){
            let reflected = ray_in.direction().reflect(hit_data.normal()).normalize();
            Some(ScatterData::new(Box::new(Color::new(1.0,1.0,1.0)), Box::new(Ray::new_with_time(hit_data.position(), reflected, ray_in.time()))))
        } else {
            let refracted = Vec3::refract(unit_direction, hit_data.normal(), ri);
            Some(ScatterData::new(Box::new(Color::new(1.0,1.0,1.0)), Box::new(Ray::new_with_time(hit_data.position(), refracted, ray_in.time()))))
        }


    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Dielectric::new(self.refractive_index))
    }
}

