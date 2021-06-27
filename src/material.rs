use crate::color::RgbColor;
use crate::geometry::HitRecord;
use crate::ray::Ray;
use crate::math::{RandomVec, Vec3, VecApprox};

use rand::Rng;

pub struct Lambertian {
    pub albedo: RgbColor
}

impl Lambertian {
    pub fn new(albedo: RgbColor) -> Self {
        Lambertian {
            albedo
        }
    }
}

pub struct Metal {
    pub albedo: RgbColor,
    pub fuzziness: f32
}

impl Metal {
    pub fn new(albedo: RgbColor, fuzziness: f32) -> Self {
        Metal {
            albedo,
            fuzziness: if fuzziness < 1.0 { fuzziness } else { 1.0 }
        }
    }
}

pub struct Dielectric {
    pub refractive_index: f32
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric {
            refractive_index
        }
    }

    fn reflectance(cos: f32, refractive_index: f32) -> f32 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(RgbColor, Ray)>;
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> Option<(RgbColor, Ray)> {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vec();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_rec.normal;
        }
        let scattered_ray = Ray::new(hit_rec.p, scatter_direction);
        let attentuation = self.albedo;
        Some((attentuation, scattered_ray))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(RgbColor, Ray)> {
        let reflected = ray.direction.normalized().reflected(hit_rec.normal);
        let scattered_ray = Ray::new(hit_rec.p, reflected + self.fuzziness * Vec3::random_in_unit_sphere());
        if scattered_ray.direction.dot(hit_rec.normal) > 0.0 {
            let attentuation = self.albedo;
            Some((attentuation, scattered_ray))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(RgbColor, Ray)> {
        let attentuation = RgbColor::new(1.0, 1.0, 1.0);
        let ref_ratio = if hit_rec.front_face { 1.0 / self.refractive_index } else { self.refractive_index };
        let unit_direction = ray.direction.normalized();

        let cos_theta = -unit_direction.dot(hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();
        let direction;
        if ref_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, ref_ratio) > rng.gen() {
            // Cannot refract, must reflect instead
            direction = unit_direction.reflected(hit_rec.normal);
        } else {
            direction = unit_direction.refracted(hit_rec.normal, ref_ratio);
        }
        let scattered_ray = Ray::new(hit_rec.p, direction);
        Some((attentuation, scattered_ray))
    }
}