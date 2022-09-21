use serde::Deserialize;

use super::{
    ray::Ray,
    utils::random_in_range,
    v3::{P3, V3},
};

#[derive(Copy, Clone, Deserialize)]
pub struct Camera {
    pub look_from: P3,
    pub look_at: P3,
    pub view_up: V3,
    pub vertical_field_of_view: f32,
    pub aspect_ratio: f32,
    pub aperture: f32,
    pub focus_distance: f32,
    pub time_start: f32,
    pub time_finish: f32,

    #[serde(skip)]
    view_width: f32,
    #[serde(skip)]
    view_height: f32,
    #[serde(skip)]
    u: V3,
    #[serde(skip)]
    v: V3,
    #[serde(skip)]
    w: V3,
    #[serde(skip)]
    horizontal: V3,
    #[serde(skip)]
    vertical: V3,
    #[serde(skip)]
    lower_left_corner: P3,
    #[serde(skip)]
    lens_radius: f32,
}

impl Camera {
    pub fn initialize(&mut self) {
        let theta = self.vertical_field_of_view.to_radians();
        let h = (theta / 2.).tan();

        let view_height = 2. * h;
        let view_width = view_height * self.aspect_ratio;

        let w = (self.look_from - self.look_at).unit();
        let u = V3::cross(&self.view_up, &w).unit();
        let v = V3::cross(&w, &u);

        let horizontal = (self.focus_distance * view_width) * u;
        let vertical = (self.focus_distance * view_height) * v;

        let lower_left_corner =
            self.look_from - horizontal / 2. - vertical / 2. - self.focus_distance * w;

        self.view_height = view_height;
        self.view_width = view_width;
        self.u = u;
        self.v = v;
        self.w = w;
        self.horizontal = horizontal;
        self.vertical = vertical;
        self.lower_left_corner = lower_left_corner;
        self.lens_radius = self.aperture / 2.;
    }

    pub fn make_ray(&self, s: f32, t: f32) -> Ray {
        // NOTE - Introduce defocus blur.

        let defocus_weights = self.lens_radius * V3::random_in_disk(1.);
        let defocus_offset = defocus_weights.x * self.u + defocus_weights.y * self.v;

        Ray {
            position: self.look_from + defocus_offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.look_from
                - defocus_offset,
            time: random_in_range(self.time_start, self.time_finish),
        }
    }
}
