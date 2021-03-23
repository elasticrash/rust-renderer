use crate::ray::Ray;
use crate::vec3::{Math, Point3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,

    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

pub trait CameraProperties {
    fn get_ray(self, u: f32, v: f32) -> Ray;
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;

        let horizontal = Vec3::new(viewport_width) * u;
        let vertical = Vec3::new(viewport_height) * v;
        let lower_left_corner = origin - horizontal / Vec3::new(2.) - vertical / Vec3::new(2.) - w;

        Self {
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
}

impl CameraProperties for Camera {
    fn get_ray(self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner
                + Vec3::new(s) * self.horizontal
                + Vec3::new(t) * self.vertical
                - self.origin,
        }
    }
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
