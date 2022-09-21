use serde::Deserialize;

use super::{camera::Camera, color::Color, object::ObjectKind};

#[derive(Clone, Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub background_color: Color,
    pub camera: Camera,
    pub root_object: ObjectKind,
}
