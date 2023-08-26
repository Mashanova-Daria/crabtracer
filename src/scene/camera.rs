use serde_json::{Value};
use crate::util::transform::Transform;
use crate::util::ray::Ray;
use crate::util::*;
use glam::f64::DVec3;

pub struct Camera {
    pub m_xform: Transform,
    pub m_size: Vec<f64>,         // physical size of image plane
    pub m_focal_distance: f64,    // distance to image plane along z axis
    pub m_resolution: Vec<i64>,   // image resolution
    pub m_aperture_radius: f64    // size of aperture for depth of field
}

impl Camera {
    // generate ray going through image-plane location (u,v)
    pub fn generate_ray(&self, u: f64, v: f64) -> Ray {
        let u_phys = u / (self.m_resolution[0] as f64);
        let v_phys = v / (self.m_resolution[1] as f64);

        let o = DVec3 { x: 0.0, y: 0.0, z: 0.0 };
        let d = DVec3 {
            x: (u_phys - 0.5) * self.m_size[0],
            y: (0.5 - v_phys) * self.m_size[1],
            z: -1.0 * self.m_focal_distance
        };

        return self.m_xform.ray(&Ray::new(o, d, None, None));
    }

    pub fn parse_from_json(j: &Value) -> Camera {
        let j = j.as_object().unwrap();

        // get transform
        let camera_transform = match j.get("transform") {
            Some(value) => Transform::from_json(value),
            None => Transform::identity()
        };

        // get values from json or defaults if none
        let fdist = match j.get("fdist") {
            Some(value) => safe_value_to_f64(value, 1.0),
            None => 1.0
        };

        let res = match j.get("resolution") {
            Some(value) => {
                match value.as_array() {
                    Some(x) => x.iter().map(|i| safe_value_to_i64(i, 512)).collect(),
                    None => vec![512, 512]
                }
            },
            None => vec![512, 512]
        };

        let aperture = match j.get("aperture") {
            Some(value) => safe_value_to_f64(value, 0.0),
            None => 0.0
        };

        // calcuate size of image plane from vfov
        let vfov = deg_2_rad(match j.get("vfov") {
            Some(value) => safe_value_to_f64(value, 90.0),
            None => 90.0
        });

        let size_y = 2.0 * (vfov / 2.0).tan() * fdist;
        let size_x = (res[0] as f64 / res[1] as f64) * size_y;

        Camera {
            m_xform: camera_transform,
            m_size: vec![size_x, size_y],
            m_focal_distance: fdist,
            m_resolution: res,
            m_aperture_radius: aperture
        }
    }
}