use std::rc::Rc;

use glam::Vec3;

use crate::{material::Material, ray::Ray};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
