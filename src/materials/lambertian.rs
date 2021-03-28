use crate::hittable::HitRecord;
use crate::materials::material::Material;
use crate::random_unit_vector;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3Attributes;

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        //println!("Lamb");

        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
            time: r_in.time
        };

        *attenuation = self.albedo;
        return true;
    }
}
