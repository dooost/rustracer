use rand::Rng;
use std::sync::Arc;

use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::math::{RandomVec, Vec3};
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
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
            return None;
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

        let material = Arc::clone(&self.material);
        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        })
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn light_test_scene() -> Self {
        let mut world = HittableList::new();

        let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
        let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
        let material_left = Arc::new(Dielectric::new(1.5));
        let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1));

        world.add(Arc::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, material_ground.clone())));
        world.add(Arc::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5, material_center.clone())));
        world.add(Arc::new(Sphere::new(Vec3::new(1.0,0.0,-1.0), 0.5, material_right.clone())));
        world.add(Arc::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, material_left.clone())));
        world.add(Arc::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0), -0.4, material_left.clone())));

        world
    }

    pub fn sample_scene() -> Self {
        let mut world = HittableList::new();

        // Ground
        let ground_mtl = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_mtl.clone(),
        )));

        let mut rng = rand::thread_rng();
        for a in -11..11 {
            for b in -11..11 {
                let sphere_radius = 0.2;
                let sphere_center = Vec3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    sphere_radius,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );
                let choose_mtl: f32 = rng.gen();

                if (sphere_center - Vec3::new(4.0, 0.2, 0.0)).mag() <= 0.9 {
                    continue;
                }

                if choose_mtl < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_mtl = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(
                        sphere_center,
                        sphere_radius,
                        sphere_mtl.clone(),
                    )));
                } else if choose_mtl < 0.95 {
                    // metal
                    let albedo = Vec3::random_bounded(0.5, 1.0);
                    let fuzziness = rng.gen_range(0.0..0.5);
                    let sphere_mtl = Arc::new(Metal::new(albedo, fuzziness));
                    world.add(Arc::new(Sphere::new(
                        sphere_center,
                        sphere_radius,
                        sphere_mtl.clone(),
                    )));
                } else {
                    let sphere_mtl = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(
                        sphere_center,
                        sphere_radius,
                        sphere_mtl.clone(),
                    )));
                }
            }
        }

        let material_dielectric = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            material_dielectric.clone(),
        )));

        let material_lambertian = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            material_lambertian.clone(),
        )));

        let material_metal = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
        world.add(Arc::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            material_metal.clone(),
        )));

        world
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
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
