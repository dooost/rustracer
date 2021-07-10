use crate::math::{f32x8, Vec3, Vec3x8};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub struct Ray8 {
    pub origin: Vec3x8,
    pub direction: Vec3x8,
}

impl Ray8 {
    pub fn new(origin: Vec3x8, direction: Vec3x8) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32x8) -> Vec3x8 {
        self.origin + t * self.direction
    }
}
