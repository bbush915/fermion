use serde::Deserialize;

use crate::raytracer::ray::Ray;

use super::{Hit, Object, ObjectKind};

#[derive(Clone, Deserialize)]
pub struct CollectionObject {
    pub objects: Vec<ObjectKind>,
}

impl Object for CollectionObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if self.objects.is_empty() {
            return None;
        }

        let mut closest_hit: Option<Hit> = None;

        let mut did_hit = false;
        let mut closest_t = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_t) {
                did_hit = true;
                closest_t = hit.t;

                closest_hit = Some(hit);
            }
        }

        if !did_hit {
            return None;
        }

        Some(closest_hit.unwrap())
    }
}
