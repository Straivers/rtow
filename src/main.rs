mod bmp;
mod camera;
mod color;
mod hittable;
mod image;
mod math;
mod ray;

use std::time::Instant;

use camera::Camera;
use color::{RgbF32, RgbU8};
use hittable::{Hittable, Sphere};
use math::{float3, Point};
use rand::random;
use ray::{HitTest, Ray};

fn ray_color(ray: &Ray, hittables: &[Hittable]) -> RgbF32 {
    if let Some(hit) = hittables.test(ray, 0.0, f32::INFINITY) {
        let color = 0.5 * (hit.normal + 1.0);
        return RgbF32::new(color.x(), color.y(), color.z());
    }

    let dir = ray.direction.normalized();
    let t = 0.5 * (dir.y() + 1.0);
    let c = (1.0 - t) * float3::new(1.0, 1.0, 1.0) + t * float3::new(0.5, 0.7, 1.0);
    RgbF32::new(c.x(), c.y(), c.z())
}

fn main() {
    // Image
    let width = 720;
    let height = 720;

    #[cfg(debug_assertions)]
    let samples = 16;
    #[cfg(not(debug_assertions))]
    let samples = 100;

    // World
    let world = vec![
        Hittable::Sphere(Sphere {
            center: Point::new(0.0, 0.0, -1.0),
            radius: 0.4,
        }),
        Hittable::Sphere(Sphere {
            center: Point::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    // Camera
    let camera = Camera::new(width, height);
    let mut pixels = Vec::new();

    let start_time = Instant::now();

    for j in 0..height {
        for i in 0..width {
            let mut color = RgbF32::BLACK;
            for _ in 0..samples {
                let u = (i as f32 + random::<f32>()) / (width as f32 - 1.0);
                let v = (j as f32 + random::<f32>()) / (height as f32 - 1.0);

                let ray = camera.get(u, v);
                color += ray_color(&ray, &world);
            }
            color /= samples as f32;
            pixels.extend_from_slice(&RgbU8::from(&color).as_u8());
        }
    }

    let end_time = Instant::now();

    println!("Render Time: {:#?}", end_time.duration_since(start_time));

    let image = image::Image::new(width, height, image::Format::RgbU8, pixels);
    let mut buffer = Vec::new();
    bmp::encode(&image, &mut buffer);

    std::fs::write("image.bmp", buffer).unwrap();
}
