use crate::util::*;
use serde_json::{Value};
use crate::util::transform::Transform;
use crate::surfaces::SurfaceBase;
use glam::DVec3;
use crate::material::{ObjectMaterial, Material};
use std::rc::Rc;

pub struct Quad {
    m_size: f64,
    m_xform: Transform,  // local to world
    m_material: Rc<ObjectMaterial>
}

impl SurfaceBase for Quad {
    fn intersect(&self, ray: &mut ray::Ray, hit: &mut ray::HitInfo) -> bool {
        let t_ray = self.m_xform.inverse().ray(ray);
        if t_ray.d.z == 0.0 {
            return false;
        };
        let t = -t_ray.o.z / t_ray.d.z;
        let mut p = t_ray.get_point(t);

        if self.m_size < p.x || -self.m_size > p.x || self.m_size < p.y || -self.m_size > p.y {
            return false;
        };

        // check ray bounds
        if t < t_ray.mint || t > t_ray.maxt {
            return false;
        };

        // project onto plane
        p.z = 0.0;

        // get normal
        let norm = self.m_xform.normal(DVec3 { x: 0.0, y: 0.0, z: -1.0 });

        // return true, record values
        hit.t = t;
        hit.p = self.m_xform.point(p);
        hit.sn = norm;
        hit.mat = Rc::clone(&self.m_material);

        return true;
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

        let material = match j.get("material") {
            Some(v) => ObjectMaterial::from_json(v),
            None => panic!("can't parse without material")
        };

        Quad { m_size: size / 2.0, m_xform: transform, m_material: Rc::new(material) }
    }
}