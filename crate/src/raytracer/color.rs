use std::ops;

use serde::Deserialize;

use super::v3::V3;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn as_v3(&self) -> V3 {
        V3 {
            x: self.r,
            y: self.g,
            z: self.b,
        }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
