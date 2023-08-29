use crate::util::*;
use crate::material::{ObjectMaterial};
use glam::f64::DVec3;
use std::rc::Rc;

pub struct Ray {
    pub o: DVec3,    // origin of ray
    pub d: DVec3,    // direction of ray
    pub mint: f64,      // min distance along ray segment
    pub maxt: f64       // max distance along ray segment
}

impl Ray {
    // get point along ray at distance t
    pub fn get_point(&self, t: f64) -> DVec3 {
        self.o + self.d * t
    }

    pub fn new(origin: DVec3, direction: DVec3, mint: Option<f64>, maxt: Option<f64>) -> Ray {
        let t_min = match mint {
            Some(x) => x,
            None => EPSILON
        };
        let t_max = match maxt {
            Some(x) => x,
            None => f64::INFINITY
        };

        Ray { o: origin, d: direction, mint: t_min, maxt: t_max }
    }
}

// strores infromation about ray-surface intersection
pub struct HitInfo {
    pub t: f64,                 // ray parameter for hit
    pub p: DVec3,               // hit position
    pub sn: DVec3,              // shading normal
    pub mat: Rc<ObjectMaterial>     // material at hit point
}

impl HitInfo {
    pub fn new() -> HitInfo {
        HitInfo { t: 0.0, p: DVec3::ZERO, sn: DVec3::ZERO, mat: Rc::new(ObjectMaterial::EMPTY) }
    }
}