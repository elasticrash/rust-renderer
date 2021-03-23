use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

pub trait CameraProperties {
    fn get_ray(self, u: f32, v: f32) -> Ray;
}

impl Camera {
    pub fn new(aspect_ratio: f32, viewport_height: f32, focal_length: f32) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Point3::new(0.);
        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.,
            z: 0.,
        };
        let vertical = Vec3 {
            x: 0.,
            y: viewport_height,
            z: 0.,
        };

        Self {
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin
                - (horizontal / Vec3::new(2.))
                - (vertical / Vec3::new(2.))
                - Vec3 {
                    x: 0.,
                    y: 0.,
                    z: focal_length,
                },
        }
    }
}

impl CameraProperties for Camera {
    fn get_ray(self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner
                + Vec3::new(u) * self.horizontal
                + Vec3::new(v) * self.vertical
                - self.origin,
        }
    }
}
