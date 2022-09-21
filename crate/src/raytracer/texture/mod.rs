use serde::Deserialize;

use self::solid::SolidTexture;

use super::{color::Color, v3::P3};

mod solid;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TextureKind {
    Solid(SolidTexture),
}

pub trait Texture {
    fn value(&self, u: f32, v: f32, position: P3) -> Color;
}

impl Texture for TextureKind {
    fn value(&self, u: f32, v: f32, position: P3) -> Color {
        match self {
            TextureKind::Solid(t) => t.value(u, v, position),
        }
    }
}
