use crate::util::*;
use serde_json::{Value};
use crate::util::transform::Transform;
use crate::surfaces::SurfaceBase;

pub struct Sphere {
    m_radius: f64,
    m_xform: Transform  // local to world
}

impl SurfaceBase for Sphere {
    fn intersect(&self, ray: &mut ray::Ray, hit: &mut ray::HitInfo) -> bool {
        let t_ray = self.m_xform.inverse().ray(ray);

        let a = t_ray.d.dot(t_ray.d);
        let b = 2.0 * t_ray.d.dot(t_ray.o);
        let c = t_ray.o.dot(t_ray.o) - self.m_radius * self.m_radius;

        // solve quadtratic
        let discrim = b*b - 4.0*a*c;
        if discrim < 0.0 {
            return false
        };

        let root_discrim = discrim.sqrt();
        let q = if b < 0.0 { -0.5 * (b - root_discrim) } else { -0.5 * (b + root_discrim) };

        let t1 = q / a;
        let t2 = c / q;

        // compute distance along ray
        let t = if t1 < t_ray.mint { t2 } else { t1 };

        // check if t is within ray limits
        if t < t_ray.mint || t > t_ray.maxt {
            return false
        };

        // get point and fill in hit info
        let p = t_ray.get_point(t) * (self.m_radius / t_ray.get_point(t).length());

        hit.t = t;
        hit.p = self.m_xform.point(p);

        return true;

    }
}

impl Sphere {
    pub fn from_json(j: &Value) -> Sphere {
        let j = j.as_object().unwrap();

        let radius = match j.get("radius") {
            Some(v) => safe_value_to_f64(v, 1.0),
            None => 1.0
        };

        let transform = match j.get("transform") {
            Some(v) => Transform::from_json(v),
            None => Transform::identity()
        };

        Sphere { m_radius: radius, m_xform: transform }
    }
}