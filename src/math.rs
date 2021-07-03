pub use ultraviolet::Vec3;

use rand::Rng;

pub trait VecApprox {
    fn is_near_zero(&self) -> bool;
}

impl VecApprox for Vec3 {
    fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

pub trait RandomVec {
    fn random() -> Self;
    fn random_bounded(min: f32, max: f32) -> Self;
    fn random_in_unit_sphere() -> Self;
    fn random_in_unit_disk() -> Self;
    fn random_unit_vec() -> Self;
}

impl RandomVec for Vec3 {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    fn random_bounded(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max), 
            rng.gen_range(min..max), 
            rng.gen_range(min..max)
        )
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_bounded(-1.0,1.0);
            if p.mag_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.mag_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    fn random_unit_vec() -> Self {
        Self::random_in_unit_sphere().normalized()
    }
}