use serde::Deserialize;

use crate::raytracer::{
    object::Hit,
    ray::Ray,
    texture::{Texture, TextureKind},
    v3::V3,
};

use super::{Material, ScatterResult};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct LambertianMaterial {
    pub texture: TextureKind,
}

impl Material for LambertianMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let mut direction = hit.normal + V3::random_in_sphere(1.);

        if direction.is_near_zero() {
            direction = hit.normal;
        }

        Some(ScatterResult {
            ray_out: Ray {
                position: hit.position,
                direction,
                time: ray_in.time,
            },
            attenuation: self.texture.value(hit.u, hit.v, hit.position),
        })
    }
}
