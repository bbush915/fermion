use super::v3::{P3, V3};

pub struct Ray {
    pub position: P3,
    pub direction: V3,
    pub time: f32,
}

impl Ray {
    pub fn at(&self, t: f32) -> P3 {
        self.position + t * self.direction
    }
}
