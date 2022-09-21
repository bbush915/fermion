use std::f32::consts::PI;

use serde::Deserialize;

use crate::raytracer::{
    material::MaterialKind,
    ray::Ray,
    v3::{P3, V3},
};

use super::{Hit, Object};

#[derive(Copy, Clone, Deserialize)]
pub struct SphereObject {
    pub position_start: P3,
    pub position_finish: P3,
    pub time_start: f32,
    pub time_finish: f32,
    pub radius: f32,
    pub material: MaterialKind,
}

impl SphereObject {
    fn get_position(&self, t: f32) -> P3 {
        // NOTE - Linear interpolation of the sphere position at the given time.

        let coefficient = (t - self.time_start) / (self.time_finish - self.time_start);

        self.position_start + coefficient * (self.position_finish - self.position_start)
    }

    fn get_uv(position: &P3) -> (f32, f32) {
        // NOTE - Spherical coordinates.

        let theta = (-position.y).acos();
        let phi = (-position.z).atan2(position.x) + PI;

        (phi / (2. * PI), theta / PI)
    }
}

impl Object for SphereObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let position = self.get_position(ray.time);
        let normal = ray.position - position;

        let a = ray.direction.len2();
        let b = 2. * V3::dot(&normal, &ray.direction);
        let c = normal.len2() - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return None;
        }

        let mut t = (-b - discriminant.sqrt()) / (2. * a);

        if t < t_min || t > t_max {
            t = (-b + discriminant.sqrt()) / (2. * a);

            if t < t_min || t > t_max {
                return None;
            }
        }

        let hit_position = ray.at(t);
        let outward_normal = (1. / self.radius) * (hit_position - position);
        let is_front = V3::dot(&ray.direction, &outward_normal) < 0.;

        let normal = if is_front {
            outward_normal
        } else {
            -outward_normal
        };

        let (u, v) = SphereObject::get_uv(&outward_normal);

        Some(Hit {
            t,
            position: hit_position,
            normal,
            is_front,
            material: self.material,
            u,
            v,
        })
    }
}
