use crate::camera::{Camera, CameraProperties};
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::materials::dielectric::Dialectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::Material;
use crate::materials::metal::Metal;
use crate::primitives::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Math;
use crate::vec3::Point3;
use crate::vec3::ToColor;
use crate::vec3::Vec3;
use crate::vec3::Vec3Attributes;
use rand::prelude::*;

use image::{imageops, ImageBuffer, RgbImage};
mod camera;
mod hittable;
mod materials;
mod primitives;
mod ray;
mod vec3;

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let samples_per_pixel = 50;
    let depth: i32 = 50;

    //World
    let mut world: Vec<(Box<dyn Hittable>, Box<dyn Material>)> =
        Vec::<(Box<dyn Hittable>, Box<dyn Material>)>::new();

    let center = Sphere::new(
        Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        0.5,
    );

    world.push((
        Box::new(center),
        Box::new(Lambertian {
            albedo: Color {
                x: 0.1,
                y: 0.2,
                z: 0.5,
            },
        }),
    ));

    let left_a = Sphere::new(
        Point3 {
            x: -1.,
            y: 0.,
            z: -1.,
        },
        0.5,
    );

    let left_b = Sphere::new(
        Point3 {
            x: -1.,
            y: 0.,
            z: -1.,
        },
        -0.45,
    );
    world.push((Box::new(left_a), Box::new(Dialectric { ir: 1.5 })));
    world.push((Box::new(left_b), Box::new(Dialectric { ir: 1.5 })));

    let right = Sphere::new(
        Point3 {
            x: 1.,
            y: 0.,
            z: -1.,
        },
        0.5,
    );

    world.push((
        Box::new(right),
        Box::new(Metal {
            albedo: Color {
                x: 0.8,
                y: 0.6,
                z: 0.2,
            },
            fuzz: 0.,
        }),
    ));

    let ground = Sphere::new(
        Point3 {
            x: 0.,
            y: -100.5,
            z: 1.,
        },
        100.,
    );

    world.push((
        Box::new(ground),
        Box::new(Lambertian {
            albedo: Color {
                x: 0.8,
                y: 0.8,
                z: 0.0,
            },
        }),
    ));

    let camera: Camera = Camera::new(
        Vec3 {
            x: -2.,
            y: 2.,
            z: 1.,
        },
        Vec3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        20.,
        aspect_ratio,
    );

    println!("P3 {} {} {:?}", img.width(), img.height(), camera);
    let mut rng = rand::thread_rng();

    for j in 0..image_height {
        if j % 50 == 0 {
            println!("{}", j);
        }
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);

            let mut pixel_color = Color {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            for _s in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen_range(0. ..1.)) / (image_width - 1) as f32;
                let v = (j as f32 + rng.gen_range(0. ..1.)) / (image_height - 1) as f32;
                let r = &camera.get_ray(u, v);
                let t_world = world.clone();
                pixel_color += ray_color(*r, t_world, depth);
            }

            let pixel_value = pixel_color.to_color(vec![
                1. / samples_per_pixel as f32,
                1. / samples_per_pixel as f32,
                1. / samples_per_pixel as f32,
            ]);
            *pixel = image::Rgb([pixel_value.r, pixel_value.g, pixel_value.b]);
        }
    }
    let subimg = imageops::flip_horizontal(&imageops::rotate180(&mut img));
    subimg.save("render.png").unwrap();
}

fn ray_color(
    ray: Ray,
    world: Vec<(Box<(dyn Hittable + 'static)>, Box<(dyn Material + 'static)>)>,
    depth: i32,
) -> Color {
    let mut rec = &mut HitRecord {
        p: Vec3::new(0.),
        normal: Vec3::new(0.),
        t: 0.,
        front_face: false,
        material_index: 0,
    };

    if depth <= 0 {
        return Color {
            x: 0.,
            y: 0.,
            z: 0.,
        };
    }
    let t_world = world.clone();

    if t_world.hit(&ray, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new(0.);
        let mat = &t_world[rec.material_index as usize].1;
        if mat.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }

        return Color::new(0.);
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.);
    Color::new(1. - t) * Color::new(1.)
        + Color::new(t)
            * Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            }
}

impl<'a> Hittable for Vec<(Box<dyn Hittable>, Box<dyn Material>)> {
    fn hit(&self, r: &ray::Ray, min: f32, max: f32, rec_out: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_so_far = max;

        let mut temp_rec: HitRecord = HitRecord {
            p: Vec3::new(0.),
            normal: Vec3::new(0.),
            t: 0.,
            front_face: false,
            material_index: 0,
        };

        for (i, (item, _material)) in self.iter().enumerate() {
            if item.hit(r, min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec_out.t = temp_rec.t;
                rec_out.p = temp_rec.p;
                rec_out.front_face = temp_rec.front_face;
                rec_out.normal = temp_rec.normal;
                rec_out.material_index = i as i32;
            }
        }

        return hit_anything;
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_rng(-1., 1.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

fn random_unit_vector() -> Vec3 {
    return random_in_unit_sphere().unit();
}
