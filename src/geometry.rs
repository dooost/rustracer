use std::rc::Rc;

use crate::math::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material
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
        if ray.direction.dot(outwards_normal) > 0.0 {
            // ray is inside the sphere
            normal = -outwards_normal;
            front_face = false;
        } else {
            // ray is outside the sphere
            normal = outwards_normal;
            front_face = true;
        }

        let material = Rc::clone(&self.material);
        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            material
        })
    }
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![],
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_yet = t_max;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_yet) {
                closest_yet = record.t;
                result = Some(record);
            }
        }
        
        result
    }
}