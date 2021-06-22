use crate::geometry::HitRecord;
use crate::ray::Ray;
use crate::math::{RandomVec, Vec3, VecApprox};

pub struct Lambertian {
    pub albedo: Vec3
}

pub struct Metal {
    pub albedo: Vec3
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
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
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction.normalized().reflected(hit_rec.normal);
        let scattered_ray = Ray::new(hit_rec.p, reflected);
        if scattered_ray.direction.dot(hit_rec.normal) > 0.0 {
            let attentuation = self.albedo;
            Some((attentuation, scattered_ray))
        } else {
            None
        }
    }
}