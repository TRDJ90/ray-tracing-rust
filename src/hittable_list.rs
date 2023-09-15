use crate::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_hit_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                temp_hit_record = Some(hit_record);
            }
        }

        temp_hit_record
    }
}
