use crate::util::ray::HitInfo;
use crate::util::*;
use image::Rgb;
use serde_json::{Value};

pub trait Texture {
    fn value(&self, hit: &HitInfo) -> Rgb<u8>; 
}

pub struct ConstantTexture {
    pub color: Rgb<u8>
}

impl ConstantTexture {
    pub fn from_json(v: &Value) -> ConstantTexture {
        ConstantTexture { color: safe_value_to_color(v, Rgb([0, 0, 0])) }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, hit: &HitInfo) -> Rgb<u8> {
        self.color
    }
}