extern crate image as imagers;

mod image;
mod math;
mod color;
mod ray;
mod geometry;
mod camera;

use math::RandomVec;
use rand::Rng;

use std::rc::Rc;

use crate::image::Image;
use camera::Camera;
use geometry::Hittable;
use geometry::HittableList;
use geometry::Sphere;
use math::Vec3;
use color::RgbColor;
use ray::Ray;

fn main() {
    // Image
    let image_height = 720;
    let image_width = 1280;
    let mut img = Image::new(image_width, image_height);

    let samples_per_pixel = 64;
    let max_depth = 32;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    let mut rng = rand::thread_rng();
    for i in (0..image_width).rev() {
        for j in 0..image_height {
            let mut color = RgbColor::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u_jitter: f32 = rng.gen();
                let u = ((i as f32) + u_jitter) / (image_width - 1) as f32;
                let v_jitter: f32 = rng.gen();
                let v = ((j as f32) + v_jitter) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, max_depth);
            }

            // The image crate's coordinate system starts from the top left corner,
            // but ours is from the bottom left, so we need to flip it vertically
            let j_in_image_coords = image_height - j - 1;
            img.set_color_at(i, j_in_image_coords, color, samples_per_pixel);
        }
    }

    img.write_png("output.png");
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: u32) -> RgbColor {
    if depth <= 0 {
        return RgbColor::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray, 0.001, f32::INFINITY) {
        let target = record.p + record.normal + Vec3::random_unit_vec();
        let p_to_target = Ray::new(record.p, target - record.p);
        return 0.5 * ray_color(&p_to_target, world, depth - 1);
    }

    let normalized_direction = ray.direction.normalized();
    let t = 0.5 * (normalized_direction.y + 1.0);
    
    (1.0 - t) * RgbColor::new( 1.0, 1.0, 1.0) + t * RgbColor::new(0.5, 0.7, 1.0)
}