use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(0.),
            direction: Vec3::new(0.),
            time: 0.
        }
    }
}

pub trait RayProperties {
    fn at(self, t: f32) -> Point3;
}

impl RayProperties for Ray {
    fn at(self, t: f32) -> Vec3 {
        self.origin + Vec3::new(t) * self.direction
    }
}
