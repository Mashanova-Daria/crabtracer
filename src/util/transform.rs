use glam::f64::{DMat4, DVec3};
use serde_json::{Value};
use crate::util::*;
use crate::util::ray::Ray;

pub struct Transform {
    pub m: DMat4,
    pub m_inv: DMat4
}

impl Transform {

    // transform vector
    pub fn vector(&self, v: DVec3) -> DVec3 {
        self.m.transform_vector3(v)
    }

    // transform point
    pub fn point(&self, p: DVec3) -> DVec3 {
        self.m.transform_point3(p)
    }

    // transform normal
    pub fn normal(&self, n: DVec3) -> DVec3 {
        self.m_inv.transpose().transform_vector3(n).normalize()
    }

    // transform ray
    pub fn ray(&self, r: &Ray) -> Ray {
        Ray::new(self.point(r.o), self.vector(r.d), Some(r.mint), Some(r.maxt))
    }

    // return inverse transform
    pub fn inverse(&self) -> Transform {
        Transform { m: self.m_inv, m_inv: self.m }
    }

    // identity transform
    pub fn identity() -> Transform {
        Transform { m: DMat4::IDENTITY , m_inv: DMat4::IDENTITY }
    }

    pub fn from_json(j: &Value) -> Transform {
        let j = j.as_object().unwrap();
        let translation_vec = match j.get("o") {
            Some(v) => match v.as_array() {
                Some(x) => DVec3 { 
                    x: safe_value_to_f64(&x[0], 0.0), 
                    y: safe_value_to_f64(&x[1], 0.0), 
                    z: safe_value_to_f64(&x[2], 0.0)
                },
                None => DVec3::ZERO
            },
            None => DVec3::ZERO
        };

        let transform_mat = DMat4::from_translation(translation_vec);

        Transform {
            m: transform_mat,
            m_inv: transform_mat.inverse()
        }
    }
}