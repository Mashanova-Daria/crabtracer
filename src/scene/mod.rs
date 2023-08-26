mod camera;

use camera::*;
use serde_json::{Value};
use crate::surfaces::{SurfaceGroup, SurfaceBase};
use crate::util::*;
use image::{RgbImage};
use rand::prelude::*;
use crate::util::ray::{Ray, HitInfo};

pub struct Scene {
    m_camera: Camera,
    m_surface_group: SurfaceGroup,
    pub m_image_samples: i64,
    pub m_background: image::Rgb<u8>
}

impl Scene {
    pub fn parse_from_json(j: &Value) -> Scene {

        Scene { 
            m_camera: Camera::parse_from_json(&j["camera"]),
            m_surface_group: SurfaceGroup::from_json(&j["surfaces"]),
            m_image_samples: safe_value_to_i64(&j["image_samples"], 1),
            m_background: safe_value_to_color(&j["background"], image::Rgb([0, 0, 0]))
        }
    }

    fn ray_trace_color(&self, ray: &mut Ray) -> image::Rgb<u8> {
        let mut hit = HitInfo::new();

        if self.m_surface_group.intersect(ray, &mut hit) {
            // get emitted color
            // get scattered ray
            // call recursivley
            //println!("found intersection");
            return image::Rgb([255, 255, 255]);
        } else {
            // return self.m_background;
            return image::Rgb([0, 0, 0]);
        }
    }

    pub fn ray_trace_image(&self) -> RgbImage {
        // allocate image
        let width = u32::try_from(self.m_camera.m_resolution[0]).unwrap();
        let height = u32::try_from(self.m_camera.m_resolution[1]).unwrap();

        let mut image = RgbImage::new(width, height);

        // for each pixel
        for i in 0..height {
            for j in 0..width {

                // init pixel color
                *image.get_pixel_mut(i, j) = image::Rgb([0, 0, 0]);

                let mut acc_color:[i64; 3] = [0, 0, 0];

                // for each sample
                for s in 0..self.m_image_samples {
                    let sample = vec![random::<f64>(), random::<f64>()];
                    let mut ray = self.m_camera.generate_ray(
                        (i as f64) + sample[0],
                        (j as f64) + sample[1]
                    );

                    let sample_color = self.ray_trace_color(&mut ray);

                    acc_color[0] += sample_color[0] as i64;
                    acc_color[1] += sample_color[1] as i64;
                    acc_color[2] += sample_color[2] as i64;
                }

                *image.get_pixel_mut(i, j) = image::Rgb([
                    (acc_color[0] / self.m_image_samples).try_into().unwrap(),
                    (acc_color[1] / self.m_image_samples).try_into().unwrap(),
                    (acc_color[2] / self.m_image_samples).try_into().unwrap()
                ]);
            }
        };

        return image;

    }
}