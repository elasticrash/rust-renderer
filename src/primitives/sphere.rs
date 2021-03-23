use crate::hittable::{HitRecord, Hittable, SetFaceNormal};
use crate::ray::{Ray, RayProperties};
use crate::vec3::{Math, Point3, Vec3, Vec3Attributes};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min: f32, max: f32, rec_out: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < min || max < root {
            root = (-half_b + sqrtd) / a;
            if root < min || max < root {
                return false;
            }
        }

        rec_out.t = root;
        rec_out.p = ray.at(rec_out.t);
        let outward = (rec_out.p - self.center) / Vec3::new(self.radius);
        rec_out.set_face_normal(ray, outward);
        return true;
    }
}
