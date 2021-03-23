use crate::hittable::HitRecord;
use crate::materials::material::Material;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::{Math, Vec3Attributes};

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        //println!("Metal");
        let reflected = (r_in.direction.unit()).reflect(rec.normal);
        *scattered = Ray {
            origin: rec.p,
            direction: reflected + Color::new(self.fuzz) * random_in_unit_sphere(),
        };

        *attenuation = self.albedo;
        return true;
    }
}
