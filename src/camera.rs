use crate::math::RandomVec;
use crate::math::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        from: Vec3,
        at: Vec3,
        up: Vec3,
        vfov: f32, // Vertical fov in degrees
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (from - at).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = from - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin: from,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let point_on_lens = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * point_on_lens.x + self.v * point_on_lens.y;
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset;
        Ray::new(self.origin + offset, direction)
    }
}
