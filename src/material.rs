use glam::Vec3;

use crate::{
    common::{near_zero, random, random_unit_vector, random_vec3_in_unit_sphere, reflect, refract},
    hittable::HitRecord,
    pixel::Pixel,
    ray::Ray,
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool;

    fn reflectance(cosine: f32, ref_idx: f32) -> f32
    where
        Self: Sized,
    {
        // Use Schlicks approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
    }
}

pub struct Lambertian {
    albedo: Pixel,
}

impl Lambertian {
    pub fn new(albedo: Pixel) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(hit_record.point, scatter_direction);
        true
    }
}

pub struct Metal {
    albedo: Pixel,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Pixel, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray_in.direction().normalize(), hit_record.normal);

        *attenuation = self.albedo;
        *scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_vec3_in_unit_sphere(),
        );
        Vec3::dot(scattered.direction(), hit_record.normal) > 0.0
    }
}

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Pixel,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().normalize();
        let cos_theta = f32::min(-unit_direction.dot(hit_record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                reflect(unit_direction, hit_record.normal)
            } else {
                refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        *attenuation = Pixel::ONE;
        *scattered = Ray::new(hit_record.point, direction);
        true
    }
}
