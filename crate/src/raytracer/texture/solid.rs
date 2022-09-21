use serde::Deserialize;

use crate::raytracer::{color::Color, v3::P3};

use super::Texture;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct SolidTexture {
    pub color: Color,
}

impl Texture for SolidTexture {
    fn value(&self, _u: f32, _v: f32, _position: P3) -> Color {
        self.color
    }
}
