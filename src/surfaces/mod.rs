mod sphere;
mod quad;

use serde_json::{Value};
use sphere::Sphere;
use quad::Quad;
use crate::util::ray::{Ray, HitInfo};

// enum for all surfaces
enum Surface {
    SURFACEGROUP(SurfaceGroup),
    SPHERE(Sphere),
    QUAD(Quad)
}

// base trait for all surfaces
pub trait SurfaceBase {
    fn is_emissive(&self) -> bool { false }
    fn intersect(&self, ray: &mut Ray, hit: &mut HitInfo) -> bool;
}

// collection of surfaces grouped together
pub struct SurfaceGroup {
    m_surfaces: Vec<Surface>
}

impl SurfaceBase for SurfaceGroup {
    fn intersect(&self, ray: &mut Ray, hit: &mut HitInfo) -> bool {
        let mut hit_something = false;

        for surface in self.m_surfaces.iter() {
            if surface.intersect(ray, hit) {
                hit_something = true;
                ray.mint = hit.t;
            }
        }

        return hit_something;
    }
}

impl SurfaceBase for Surface {
    fn intersect(&self, ray: &mut Ray, hit: &mut HitInfo) -> bool {
        match self {
            Surface::SURFACEGROUP(s) => s.intersect(ray, hit),
            Surface::SPHERE(s) => s.intersect(ray, hit),
            Surface::QUAD(s) => s.intersect(ray, hit)
        }
    }
}

impl SurfaceGroup {
    // create surface group from json
    pub fn from_json(j: &Value) -> SurfaceGroup {
        let surface_array = match j.as_array() {
            Some(v) => v,
            None => return SurfaceGroup { m_surfaces: Vec::new() }
        };
        let mut surface_agg:Vec<Surface> = Vec::new();

        for v in surface_array.iter() {
            let surface_object = v.as_object().unwrap();
            let surface_type = surface_object.get("type").unwrap();

            if surface_type == "sphere" {
                surface_agg.push(Surface::SPHERE(Sphere::from_json(v)));
            };
            if surface_type == "quad" {
                surface_agg.push(Surface::QUAD(Quad::from_json(v)));
            };
        }

        SurfaceGroup {
            m_surfaces: surface_agg
        }
    }

}