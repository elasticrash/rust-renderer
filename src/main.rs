use crate::camera::{Camera, CameraProperties};
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::materials::dielectric::Dialectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::Material;
use crate::materials::metal::Metal;
use crate::primitives::moving_sphere::MovingSphere;
use crate::primitives::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Math;
use crate::vec3::Point3;
use crate::vec3::ToColor;
use crate::vec3::Vec3;
use crate::vec3::Vec3Attributes;
use rand::prelude::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
extern crate num_cpus;

use image::{imageops, ImageBuffer, RgbImage};
mod camera;
mod hittable;
mod materials;
mod primitives;
mod ray;
mod vec3;

fn main() {
    //Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 600;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let samples_per_pixel = 50;
    let depth: i32 = 50;

    println!("using {} threads", num_cpus::get());

    //World
    let mut world: Vec<(
        Arc<dyn Hittable + Sync + Send>,
        Arc<dyn Material + Sync + Send>,
    )> = Vec::<(
        Arc<dyn Hittable + Sync + Send>,
        Arc<dyn Material + Sync + Send>,
    )>::new();

    let ground = Sphere::new(
        Point3 {
            x: 0.,
            y: -1000.,
            z: 0.,
        },
        1000.,
    );

    world.push((
        Arc::new(ground),
        Arc::new(Lambertian {
            albedo: Color {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        }),
    ));

    let mut rng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let mat = rng.gen_range(0. ..1.);
            let center = Point3 {
                x: (i as f32) + 0.9 * rng.gen_range(0. ..1.),
                y: 0.2,
                z: (j as f32) + 0.9 * rng.gen_range(0. ..1.),
            };

            if (center
                - Point3 {
                    x: 4.,
                    y: 0.2,
                    z: 0.,
                })
            .length()
                > 0.9
            {
                if mat < 0.8 {
                    let center2 = center
                        + Vec3 {
                            x: 0.,
                            y: rng.gen_range(0. ..0.5),
                            z: 0.,
                        };

                    let object = MovingSphere::new(center, center2, 0., 1., 0.2);

                    world.push((
                        Arc::new(object),
                        Arc::new(Lambertian {
                            albedo: Color::random() * Color::random(),
                        }),
                    ));
                } else if mat < 0.95 {
                    let object = Sphere::new(center, 0.2);

                    world.push((
                        Arc::new(object),
                        Arc::new(Metal {
                            albedo: Color::random() * Color::random(),
                            fuzz: rng.gen_range(0. ..0.5),
                        }),
                    ));
                } else {
                    let object = Sphere::new(center, 0.2);
                    world.push((Arc::new(object), Arc::new(Dialectric { ir: 1.5 })));
                }
            }
        }
    }

    let one = Sphere::new(
        Point3 {
            x: -4.,
            y: 1.,
            z: 0.,
        },
        1.,
    );

    world.push((
        Arc::new(one),
        Arc::new(Lambertian {
            albedo: Color {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        }),
    ));

    let two = Sphere::new(
        Point3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        1.,
    );
    world.push((Arc::new(two), Arc::new(Dialectric { ir: 1.5 })));

    let three = Sphere::new(
        Point3 {
            x: 4.,
            y: 1.,
            z: 0.,
        },
        1.,
    );

    world.push((
        Arc::new(three),
        Arc::new(Metal {
            albedo: Color {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.,
        }),
    ));

    let lookfrom = Vec3 {
        x: 13.,
        y: 2.,
        z: 3.,
    };

    let lookat = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let camera: Camera = Camera::new(
        lookfrom,
        lookat,
        Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        20.,
        aspect_ratio,
        0.1,
        10.,
    );

    println!("P3 {} {} {:?}", img.width(), img.height(), camera);
    let mut fake_image: Vec<Pixel> = vec![];

    let safe_world = Arc::new(world.clone());
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();

    for j in 0..image_height {
        if j % 10 == 0 {
            println!("{}", j);
        }
        for i in 0..image_width {
            let safe = Arc::clone(&safe_world);
            let tx1 = mpsc::Sender::clone(&tx);

            let handle = thread::spawn(move || {
                let mut rng = rand::thread_rng();

                let mut pixel_color = Color {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                for _s in 0..samples_per_pixel {
                    let u = (i as f32 + rng.gen_range(0. ..1.)) / (image_width - 1) as f32;
                    let v = (j as f32 + rng.gen_range(0. ..1.)) / (image_height - 1) as f32;
                    let r = &camera.get_ray(u, v);
                    pixel_color += ray_color(*r, safe.to_vec(), depth);
                }

                let pixel_value = pixel_color.to_color(vec![
                    1. / samples_per_pixel as f32,
                    1. / samples_per_pixel as f32,
                    1. / samples_per_pixel as f32,
                ]);

                tx1.send(Arc::new(Pixel {
                    x: i,
                    y: j,
                    r: pixel_value.r,
                    g: pixel_value.g,
                    b: pixel_value.b,
                }))
                .unwrap();
            });

            handles.push(handle);

            if handles.len() == num_cpus::get() {
                for h in handles {
                    h.join().unwrap();
                }
                handles = vec![];
            }
        }
    }

    for i in rx {
        fake_image.push(*i);
        if fake_image.len() == (image_height * image_width) as usize {
            break;
        }
    }

    println!("saving image");
    for p in &mut fake_image.clone() {
        let pixel = img.get_pixel_mut(p.x, p.y);
        *pixel = image::Rgb([p.r, p.g, p.b]);
    }

    let subimg = imageops::flip_horizontal(&imageops::rotate180(&mut img));
    subimg.save("render.png").unwrap();
}

fn ray_color(
    ray: Ray,
    world: Vec<(
        Arc<(dyn Hittable + Sync + Send)>,
        Arc<(dyn Material + Sync + Send)>,
    )>,
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

impl<'a> Hittable
    for Vec<(
        Arc<dyn Hittable + Sync + Send>,
        Arc<dyn Material + Sync + Send>,
    )>
{
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

#[derive(Clone, Copy, Debug)]
struct Pixel {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}
