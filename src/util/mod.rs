pub mod transform;
pub mod ray;

use serde_json::{Value};
use glam::DVec3;

pub const M_PI: f64 = 3.14159265358979323846;
pub const EPSILON: f64 = 0.001;
pub const MAX_RAYTRACE_DEPTH: i32 = 64;

// convert degrees to radians
pub fn deg_2_rad(v: f64) -> f64 { v * (M_PI / 180.0) }

// get point on unit sphere from random sample
pub fn random_on_unit_sphere(x: f64, y: f64) -> DVec3 {
    let phi = x * 2.0 * M_PI;
    let cos_theta = 2.0 * y - 1.0;
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    return DVec3 {
        x: phi.cos() * sin_theta,
        y: phi.sin() * sin_theta,
        z: cos_theta
    }
}

// convert value to f64 or default on error
pub fn safe_value_to_f64(v: &Value, default: f64) -> f64 {
    match v.as_f64() {
        Some(x) => x,
        None => default
    }
}

// convert value to i64 or default on error
pub fn safe_value_to_i64(v: &Value, default: i64) -> i64 {
    match v.as_i64() {
        Some(x) => x,
        None => default
    }
}

// convert value to color
pub fn safe_value_to_color(v: &Value, default: image::Rgb<u8>) -> image::Rgb<u8> {
    match v.as_array() {
        Some(x) => {
            image::Rgb([
                safe_value_to_i64(&x[0], 0).try_into().unwrap(),
                safe_value_to_i64(&x[1], 0).try_into().unwrap(),
                safe_value_to_i64(&x[2], 0).try_into().unwrap()
            ])
        },
        None => default
    }
}

// attenuate b by a
pub fn compose_color(a: image::Rgb<u8>, b: image::Rgb<u8>) -> image::Rgb<u8> {
    let r_att: f32 = (a[0] as f32 / 255.0) * b[0] as f32;
    let g_att: f32 = (a[1] as f32 / 255.0) * b[1] as f32;
    let b_att: f32 = (a[2] as f32 / 255.0) * b[2] as f32;

    image::Rgb([
        r_att.round() as u8,
        g_att.round() as u8,
        b_att.round() as u8
    ])
}

pub fn add_color(a: image::Rgb<u8>, b: image::Rgb<u8>) -> image::Rgb<u8> {
    image::Rgb([a[0]+b[0], a[1]+b[1], a[2]+b[2]])
}