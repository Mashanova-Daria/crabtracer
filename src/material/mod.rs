pub mod texture;

use texture::{ConstantTexture, Texture};
use image::Rgb;
use crate::util::ray::{Ray, HitInfo};
use crate::util::*;
use rand::prelude::*;
use serde_json::{Value};

pub enum ObjectMaterial {
    LAMBERTIAN(Lambertian),
    EMPTY
}

pub trait Material {
    // amount of emitted light at surface hitpoint
    fn emitted(&self, ray: &Ray, hit: &HitInfo) -> Rgb<u8>;

    // scattered direction at surface hitpoint
    // return attenuation if surface scatters light
    fn scatter(&self, ray: &Ray, hit: &HitInfo, scattered: &mut Ray) -> Option<Rgb<u8>>;

    fn from_json(j: &Value) -> Self;
}

pub struct Lambertian {
    pub albedo: ConstantTexture
}

impl Material for Lambertian {
    fn from_json(j: &Value) -> Self {
        Lambertian { albedo: ConstantTexture::from_json(j) }
    }

    fn emitted(&self, ray: &Ray, hit: &HitInfo) -> Rgb<u8> {
        image::Rgb([0, 0, 0])
    }

    fn scatter(&self, ray: &Ray, hit: &HitInfo, scattered: &mut Ray) -> Option<Rgb<u8>> {
        let attenuation = self.albedo.value(hit);

        // get scattered ray
        scattered.o = hit.p;
        scattered.d = hit.sn + random_on_unit_sphere(random::<f64>(), random::<f64>());

        return Some(attenuation);
    }
}

impl Material for ObjectMaterial {
    fn from_json(j: &Value) -> Self {
        let material_type = j.get("type").unwrap().as_str().unwrap();

        if material_type == "lambertian" {
            return ObjectMaterial::LAMBERTIAN(Lambertian::from_json(j.get("albedo").unwrap()));
        } else {
            panic!("could not parse material")
        }
    }

    fn emitted(&self, ray: &Ray, hit: &HitInfo) -> Rgb<u8> {
        match self {
            ObjectMaterial::LAMBERTIAN(v) => v.emitted(ray, hit),
            ObjectMaterial::EMPTY => Rgb([0, 0, 0])
        }
    }

    fn scatter(&self, ray: &Ray, hit: &HitInfo, scattered: &mut Ray) -> Option<Rgb<u8>> {
        match self {
            ObjectMaterial::LAMBERTIAN(v) => v.scatter(ray, hit, scattered),
            ObjectMaterial::EMPTY => None
        }
    }
}