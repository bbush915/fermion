use std::f32::INFINITY;

use self::{material::Material, object::Object, ray::Ray, v3::V3};

mod camera;
mod color;
mod material;
mod object;
mod ray;
mod scene;
mod texture;
mod utils;
mod v3;

pub use scene::Scene;

pub fn trace_ray(scene: &Scene, x: u32, y: u32) -> (u8, u8, u8) {
    let mut accumulated_color = V3::default();

    for _ in 0..scene.samples_per_pixel {
        let u = (y as f32 + rand::random::<f32>()) / (scene.width as f32 - 1.);
        let v = 1. - (x as f32 + rand::random::<f32>()) / (scene.height as f32 - 1.);

        let ray = scene.camera.make_ray(u, v);
        let sample_color = bounce_ray(scene, &ray, scene.max_depth);

        accumulated_color += sample_color;
    }

    accumulated_color.x = (accumulated_color.x / scene.samples_per_pixel as f32).sqrt();
    accumulated_color.y = (accumulated_color.y / scene.samples_per_pixel as f32).sqrt();
    accumulated_color.z = (accumulated_color.z / scene.samples_per_pixel as f32).sqrt();

    let r = (256. * accumulated_color.x.clamp(0., 0.999)) as u8;
    let g = (256. * accumulated_color.y.clamp(0., 0.999)) as u8;
    let b = (256. * accumulated_color.z.clamp(0., 0.999)) as u8;

    (r, g, b)
}

fn bounce_ray(scene: &Scene, ray: &Ray, depth: u32) -> V3 {
    if depth <= 0 {
        return V3::default();
    }

    let maybe_hit = scene.root_object.hit(ray, 0.001, INFINITY);

    if maybe_hit.is_none() {
        return scene.background_color.as_v3();
    }

    let hit = maybe_hit.unwrap();

    let emitted_color = hit.material.emit(hit.u, hit.v, hit.position);

    let maybe_scatter = hit.material.scatter(ray, &hit);

    if maybe_scatter.is_none() {
        return emitted_color.as_v3();
    }

    let scatter = maybe_scatter.unwrap();

    let scattered_color = emitted_color.as_v3()
        + V3::hadamard(
            &scatter.attenuation.as_v3(),
            &bounce_ray(scene, &scatter.ray_out, depth - 1),
        );

    scattered_color
}
