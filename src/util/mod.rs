pub mod transform;
pub mod ray;

use serde_json::{Value};

const M_PI: f64 = 3.14159265358979323846;
const EPSILON: f64 = 0.001;

// convert degrees to radians
pub fn deg_2_rad(v: f64) -> f64 { v * (M_PI / 180.0) }

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