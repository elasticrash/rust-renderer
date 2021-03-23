use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::{Math, Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material_index: i32,
}

pub trait SetFaceNormal {
    fn set_face_normal(&mut self, ray: &Ray, outward: Vec3);
}

impl SetFaceNormal for HitRecord {
    fn set_face_normal<'a>(&mut self, ray: &Ray, outward: Vec3) {
        self.front_face = ray.direction.dot(outward) < 0.;
        self.normal = if self.front_face { outward } else { -outward };
    }
}

pub trait Hittable: HitClone {
    fn hit<'a>(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

pub trait HitClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> HitClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}
