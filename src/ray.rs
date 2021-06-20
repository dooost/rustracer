use crate::math::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn from(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}