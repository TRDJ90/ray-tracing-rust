use glam::Vec3;

use crate::{
    common::{degrees_to_radian, random_in_unit_disk},
    ray::Ray,
};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vertical_field_of_view: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let aspect_ratio = width as f32 / height as f32;
        let theta = degrees_to_radian(vertical_field_of_view);
        let h = f32::tan(theta / 2.0);

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = (view_up.cross(w)).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * -viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
