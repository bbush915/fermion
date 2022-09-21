use serde::Deserialize;

use self::{dialectric::DialectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial};

use super::{color::Color, object::Hit, ray::Ray, v3::P3};

mod dialectric;
mod lambertian;
mod metal;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MaterialKind {
    Dialectric(DialectricMaterial),
    Lambertian(LambertianMaterial),
    Metal(MetalMaterial),
}

impl MaterialKind {
    pub fn emit(&self, _u: f32, _v: f32, _position: P3) -> Color {
        match self {
            _ => Color {
                r: 0.,
                g: 0.,
                b: 0.,
            },
        }
    }
}

pub struct ScatterResult {
    pub ray_out: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult>;
}

impl Material for MaterialKind {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult> {
        match self {
            MaterialKind::Dialectric(x) => x.scatter(ray_in, hit),
            MaterialKind::Lambertian(x) => x.scatter(ray_in, hit),
            MaterialKind::Metal(x) => x.scatter(ray_in, hit),
        }
    }
}
