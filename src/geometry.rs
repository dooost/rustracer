use crate::math::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let half_b = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
    
        if discriminant < 0.0 {
            return None
        }

        let disc_sqrt = discriminant.sqrt();
        let mut root = (-half_b - disc_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + disc_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outwards_normal = (p - self.center) / self.radius;

        let normal;
        let front_face;
        if ray.direction.dot(outwards_normal) < 0.0 {
            // ray is inside the sphere
            normal = -outwards_normal;
            front_face = false;
        } else {
            // ray is inside the sphere
            normal = outwards_normal;
            front_face = true;
        }

        Some(HitRecord {
            p,
            normal,
            t,
            front_face
        })
    }
}