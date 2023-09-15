use std::rc::Rc;

use glam::Vec3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let origin_center = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = origin_center.dot(ray.direction());
        let c = origin_center.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_discriminant) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let mut hit_record = HitRecord::new();
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = Some(self.material.clone());

        Some(hit_record)
    }
}
