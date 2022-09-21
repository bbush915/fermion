use serde::Deserialize;

use crate::raytracer::{color::Color, object::Hit, ray::Ray, v3::V3};

use super::{Material, ScatterResult};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MetalMaterial {
    pub albedo: Color,
    pub fuzzing_factor: f32,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let direction = V3::reflect(ray_in.direction.unit(), hit.normal);
        let fuzzed_direction = direction + self.fuzzing_factor * V3::random_in_sphere(1.);

        let ray_out = Ray {
            position: hit.position,
            direction: fuzzed_direction,
            time: ray_in.time,
        };

        if V3::dot(&ray_out.direction, &hit.normal) <= 0. {
            return None;
        }

        Some(ScatterResult {
            ray_out,
            attenuation: self.albedo,
        })
    }
}
