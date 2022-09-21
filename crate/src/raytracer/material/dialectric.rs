use serde::Deserialize;

use crate::raytracer::{color::Color, object::Hit, ray::Ray, v3::V3};

use super::{Material, ScatterResult};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct DialectricMaterial {
    pub refractive_index: f32,
}

impl Material for DialectricMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let refractive_index_ratio = if hit.is_front {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction.unit();

        let cos_theta = V3::dot(&-unit_direction, &hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let is_total_internal_reflection = refractive_index_ratio * sin_theta > 1.;

        // NOTE - Use Schlick's approximation to calculate reflectance.

        let r0 = ((1. - refractive_index_ratio) / (1. + refractive_index_ratio)).powi(2);
        let reflectance = r0 + (1. - r0) * (1. - cos_theta).powi(5);

        let direction = if is_total_internal_reflection || reflectance > rand::random::<f32>() {
            V3::reflect(unit_direction, hit.normal)
        } else {
            V3::refract(unit_direction, hit.normal, refractive_index_ratio)
        };

        Some(ScatterResult {
            ray_out: Ray {
                position: hit.position,
                direction,
                time: ray_in.time,
            },
            attenuation: Color {
                r: 1.,
                g: 1.,
                b: 1.,
            },
        })
    }
}
