use serde::Deserialize;

use self::{collection::CollectionObject, sphere::SphereObject};

use super::{
    material::MaterialKind,
    ray::Ray,
    v3::{P3, V3},
};

mod collection;
mod sphere;

#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ObjectKind {
    Collection(CollectionObject),
    Sphere(SphereObject),
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

impl Object for ObjectKind {
    fn hit(&self, ray_in: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        match self {
            ObjectKind::Collection(x) => x.hit(ray_in, t_min, t_max),
            ObjectKind::Sphere(x) => x.hit(ray_in, t_min, t_max),
        }
    }
}

#[derive(Debug)]
pub struct Hit {
    pub t: f32,
    pub position: P3,
    pub normal: V3,
    pub is_front: bool,
    pub material: MaterialKind,
    pub u: f32,
    pub v: f32,
}
