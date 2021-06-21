extern crate image as imagers;

mod image;
mod math;
mod color;
mod ray;
mod geometry;

use std::rc::Rc;

use crate::image::Image;
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
    let aspect_ratio = (image_width as f32) / (image_height as f32);
    let mut img = Image::new(image_width, image_height);

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    for i in (0..image_width).rev() {
        for j in 0..image_height {
            let u = (i as f32) / (image_width - 1) as f32;
            let v = (j as f32) / (image_height - 1) as f32;

            let ray = Ray::new(
                origin, 
                lower_left_corner + u * horizontal + v * vertical - origin
            );

            // The image crate's coordinate system starts from the top left corner,
            // but ours is from the bottom left, so we need to flip it vertically
            let j_in_image_coords = image_height - j - 1;
            img.set_color_at(i, j_in_image_coords, ray_color(&ray, &world));
        }
    }

    img.write_png("output.png");
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> RgbColor {
    if let Some(record) = world.hit(ray, 0.0, 1000000.0) {
        return 0.5 * (record.normal + RgbColor::new(1.0,1.0,1.0));
    }

    let normalized_direction = ray.direction.normalized();
    let t = 0.5 * (normalized_direction.y + 1.0);
    
    (1.0 - t) * RgbColor::new(
        1.0, 1.0, 1.0) + t * RgbColor::new(0.5, 0.7, 1.0
    )
}