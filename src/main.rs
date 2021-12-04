mod bmp;
mod color;
mod hittable;
mod image;
mod math;
mod ray;

use color::Rgb;
use hittable::{Hittable, Sphere};
use math::{float3, Point};
use ray::{HitTest, Ray};

fn ray_color(ray: &Ray, hittables: &[Hittable]) -> Rgb {
    if let Some(hit) = hittables.test(ray, 0.0, f32::INFINITY) {
        let color = 0.5 * (hit.normal + 1.0);
        return Rgb::from_f32(color.x(), color.y(), color.z());
    }

    let dir = ray.direction.normalized();
    let t = 0.5 * (dir.y() + 1.0);
    let c = (1.0 - t) * float3::new(1.0, 1.0, 1.0) + t * float3::new(0.5, 0.7, 1.0);
    Rgb::from_f32(c.x(), c.y(), c.z())
}

fn main() {
    // Image
    let aspect = 16.0 / 9.0;
    let width = 720;
    let height = (width as f32 / aspect) as u32;

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
    let viewport_height = 2.0;
    let viewport_width = aspect * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = float3::new(viewport_width, 0.0, 0.0);
    let vertical = float3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - float3::new(0.0, 0.0, focal_length);

    let mut pixels = Vec::new();

    for j in 0..height {
        for i in 0..width {
            let u = i as f32 / (width as f32 - 1.0);
            let v = j as f32 / (height as f32 - 1.0);

            let r = Ray {
                origin,
                direction: lower_left_corner.as_vec() + u * horizontal + v * vertical
                    - origin.as_vec(),
            };

            pixels.extend_from_slice(&ray_color(&r, &world).as_u8());
        }
    }

    let image = image::Image::new(width, height, image::Format::RGB8, pixels);
    let mut buffer = Vec::new();
    bmp::encode(&image, &mut buffer);

    std::fs::write("image.bmp", buffer).unwrap();
}
