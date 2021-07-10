use std::vec;

pub use ultraviolet::Vec3;
pub use ultraviolet::Vec3x8;
pub use ultraviolet::f32x8;

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
            rng.gen_range(min..max),
        )
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_bounded(-1.0, 1.0);
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

impl RandomVec for Vec3x8 {
    fn random() -> Self {
        Self::create_using_closure(Vec3::random)
    }

    fn random_bounded(min: f32, max: f32) -> Self {
        Self::create_using_closure(|| {
            Vec3::random_bounded(min, max)
        })
    }

    fn random_in_unit_sphere() -> Self {
        Self::create_using_closure(|| {
            Vec3::random_in_unit_sphere()
        })
    }

    fn random_in_unit_disk() -> Self {
        Self::create_using_closure(|| {
            Vec3::random_in_unit_disk()
        })
    }

    fn random_unit_vec() -> Self {
        Self::create_using_closure(|| {
            Vec3::random_unit_vec()
        })
    }
}

pub trait WideFromClosure<T> {
    fn create_using_closure<F>(creator: F) -> Self
    where
        F: Fn() -> T;
}

impl WideFromClosure<Vec3> for Vec3x8 {
    fn create_using_closure<F>(creator: F) -> Self
    where
        F: Fn() -> Vec3,
    {
        let mut vectors = [Vec3::zero(); 8];
        for i in 0..8 {
            vectors[i] = creator();
        }
        Vec3x8::from(vectors)
    }
}
