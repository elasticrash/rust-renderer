use crate::random_in_unit_sphere;
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
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

pub trait CameraProperties {
    fn get_ray(self, u: f32, v: f32) -> Ray;
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;

        let horizontal = Vec3::new(focus_dist) * Vec3::new(viewport_width) * u;
        let vertical = Vec3::new(focus_dist) * Vec3::new(viewport_height) * v;
        let lower_left_corner = origin
            - horizontal / Vec3::new(2.)
            - vertical / Vec3::new(2.)
            - Vec3::new(focus_dist) * w;

        let lens_radius = aperture / 2.;

        Self {
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            lens_radius: lens_radius,
            u: u,
            v: v,
            w: w,
        }
    }
}

impl CameraProperties for Camera {
    fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = Vec3::new(self.lens_radius) * random_in_unit_sphere();
        let offset = self.u * Vec3::new(rd.x) + self.v * Vec3::new(rd.y);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner
                + Vec3::new(s) * self.horizontal
                + Vec3::new(t) * self.vertical
                - self.origin
                - offset,
        }
    }
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
