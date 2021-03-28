use crate::hittable::{HitRecord, Hittable, SetFaceNormal};
use crate::ray::{Ray, RayProperties};
use crate::vec3::{Math, Point3, Vec3, Vec3Attributes};

#[derive(Copy, Clone)]
pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32) -> Self {
        Self {
            center0: center0,
            center1: center1,
            radius: radius,
            time0: time0,
            time1: time1,
        }
    }
    pub fn center(self, time: f32) -> Point3 {
        self.center0
            + Point3::new((time - self.time0) / (self.time1 - self.time0))
                * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, min: f32, max: f32, rec_out: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center(ray.time);
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
        let outward = (rec_out.p - self.center(ray.time)) / Vec3::new(self.radius);
        rec_out.set_face_normal(ray, outward);
        return true;
    }
}
