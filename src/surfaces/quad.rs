use crate::util::*;
use serde_json::{Value};
use crate::util::transform::Transform;
use crate::surfaces::SurfaceBase;

pub struct Quad {
    m_size: f64,
    m_xform: Transform
}

impl SurfaceBase for Quad {
    fn intersect(&self, ray: &mut ray::Ray, hit: &mut ray::HitInfo) -> bool {
        // TODO: implement intesect
        return false
    }
}

impl Quad {
    pub fn from_json(j: &Value) -> Quad {
        let j = j.as_object().unwrap();

        let size = match j.get("size") {
            Some(v) => safe_value_to_f64(v, 1.0),
            None => 1.0
        };

        let transform = match j.get("transform") {
            Some(v) => Transform::from_json(v),
            None => Transform::identity()
        };

        Quad { m_size: size / 2.0, m_xform: transform }
    }
}