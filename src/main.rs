extern crate image as imagers;

mod image;
mod math;
mod color;
mod ray;
mod geometry;
mod camera;
mod material;

use rand::Rng;


use crate::image::Image;
use camera::Camera;
use geometry::{Hittable, HittableList};
use math::Vec3;
use color::RgbColor;
use ray::Ray;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height = 720;
    let image_width = (image_height as f32 * aspect_ratio) as u32;
    let mut img = Image::new(image_width, image_height);

    let samples_per_pixel = 512;
    let max_depth = 50;

    // World
    let world = HittableList::sample_scene();

    // let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1));

    // world.add(Rc::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, material_ground.clone())));
    // world.add(Rc::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5, material_center.clone())));
    // world.add(Rc::new(Sphere::new(Vec3::new(1.0,0.0,-1.0), 0.5, material_right.clone())));
    // world.add(Rc::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, material_left.clone())));
    // world.add(Rc::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0), -0.4, material_left.clone())));

    // Camera
    let from = Vec3::new(13.0, 2.0, 3.0);
    let at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(from, at, up,20.0, aspect_ratio, aperture, focus_dist);

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
        match record.material.scatter(ray, &record) {
            None => return RgbColor::new(0.0, 0.0, 0.0),
            Some((attentuation, scattered_ray)) => {
                return attentuation * ray_color(&scattered_ray, world, depth - 1);
            }
        }
    }

    let normalized_direction = ray.direction.normalized();
    let t = 0.5 * (normalized_direction.y + 1.0);
    
    (1.0 - t) * RgbColor::new( 1.0, 1.0, 1.0) + t * RgbColor::new(0.5, 0.7, 1.0)
}