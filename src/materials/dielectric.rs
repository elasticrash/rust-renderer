use crate::hittable::HitRecord;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Math;
use crate::vec3::{Vec3, Vec3Attributes};
use core::cmp::Ordering::Equal;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Dialectric {
    pub ir: f32,
}

impl Material for Dialectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.unit();
        let mut cos_theta_col = [-unit_direction.dot(rec.normal), 1.];
        cos_theta_col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

        let cos_theta = cos_theta_col.first().unwrap();
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();
        let direction = if cannot_refract
            || reflectance(*cos_theta, refraction_ratio) > rng.gen_range(0. ..1.)
        {
            unit_direction.reflect(rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray {
            origin: rec.p,
            direction: direction,
        };

        return true;
    }
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let mut cos_theta_col = [-uv.dot(*n), 1.0];
    cos_theta_col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    let cos_theta = cos_theta_col.first().unwrap();
    let r_out_perp = Vec3::new(etai_over_etat) * (*uv + Vec3::new(*cos_theta) * *n);
    let r_out_parallel = Vec3::new(-((1.0 - r_out_perp.length_squared()).abs()).sqrt()) * *n;
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0.powf(2.);
    r0 + (1. - r0) * (1. - cosine).powf(0.5)
}
