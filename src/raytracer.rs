use std::rc::Rc;

use glam::Vec3;

use crate::common;
use crate::{
    camera::Camera,
    common::{random, random_range, random_vec3, random_vec3_range},
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    pixel::Pixel,
    ray::Ray,
    sphere::Sphere,
};

pub struct RayTracer {
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    camera: Camera,
    world: HittableList,
}

impl RayTracer {
    pub fn new(width: u32, height: u32, samples_per_pixel: u32, max_depth: u32) -> RayTracer {
        let world = random_scene();

        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;

        let camera = Camera::new(
            width,
            height,
            look_from,
            look_at,
            vup,
            20.0,
            aperture,
            dist_to_focus,
        );

        RayTracer {
            width,
            height,
            samples_per_pixel,
            max_depth,
            camera,
            world,
        }
    }

    pub fn render(&self, frame: &mut [u8]) {
        for j in (0..self.height as usize).rev() {
            for i in 0..self.width as usize {
                let mut pixel_color = Pixel::ZERO;
                let pixel_number = j * self.width as usize + i;
                let pixel_index = pixel_number * 4;

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + common::random()) / (self.width - 1) as f32;
                    let v = (j as f32 + common::random()) / (self.height - 1) as f32;
                    let ray = self.camera.get_ray(u, v);
                    pixel_color += self.ray_color(&ray, &self.world, self.max_depth);
                }

                self.write_pixel(&pixel_color, frame, pixel_index);
            }
        }
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList, depth: u32) -> Pixel {
        if depth <= 0 {
            return Pixel::ZERO;
        }

        if let Some(hit_record) = world.hit(ray, 0.001, common::INFINITY) {
            let mut attenuation = Pixel::default();
            let mut scattered = Ray::default();

            if hit_record.material.as_ref().unwrap().scatter(
                ray,
                &hit_record,
                &mut attenuation,
                &mut scattered,
            ) {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }
            return Pixel::ZERO;
        }

        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Pixel::ONE + t * Pixel::new(0.5, 0.7, 1.0)
    }

    fn write_pixel(&self, pixel: &Pixel, frame: &mut [u8], pixel_index: usize) {
        let mut r = pixel.x;
        let mut g = pixel.y;
        let mut b = pixel.z;

        let scale = 1.0 / self.samples_per_pixel as f32;
        r = f32::sqrt(scale * r);
        g = f32::sqrt(scale * g);
        b = f32::sqrt(scale * b);

        frame[pixel_index + 0] = (255.0 * r.clamp(0.0, 0.999)) as u8;
        frame[pixel_index + 1] = (255.0 * g.clamp(0.0, 0.999)) as u8;
        frame[pixel_index + 2] = (255.0 * b.clamp(0.0, 0.999)) as u8;
        frame[pixel_index + 3] = 255;
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Pixel::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random();
            let center: Vec3 = Vec3::new(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = random_vec3() * random_vec3();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = random_vec3_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Pixel::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Pixel::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
