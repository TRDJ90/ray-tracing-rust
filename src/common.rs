use glam::Vec3;
use rand::Rng;

use std::arch::aarch64::vsetq_lane_f32;
pub use std::f32::consts::PI;
pub use std::f32::INFINITY;

pub fn degrees_to_radian(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random() -> f32 {
    rand::thread_rng().gen::<f32>()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random(), random(), random())
}

pub fn random_vec3_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let point = random_vec3_range(-1.0, 1.0);
        if point.length_squared() >= 1.0 {
            continue;
        }
        return point;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_vec3_in_unit_sphere().normalize()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let point = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if point.length_squared() >= 1.0 {
            continue;
        }
        return point;
    }
}

pub fn near_zero(vector: &Vec3) -> bool {
    const EPS: f32 = 1.0e-8;
    vector.x.abs() < EPS && vector.y.abs() < EPS && vector.z.abs() < EPS
}

pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * Vec3::dot(vector, normal) * normal
}

pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(Vec3::dot(-uv, normal), 1.0);
    let r_out_perp = etai_over_etat * (uv * cos_theta * normal);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * normal;

    r_out_perp + r_out_parallel
}
