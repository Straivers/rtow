use crate::{
    math::{float3, Point},
    ray::Ray,
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: float3,
    vertical: float3,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect = width as f32 / height as f32;
        let viewport_height = 2.0;
        let viewport_width = aspect * viewport_height;
        let focal_length = 1.0;

        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = float3::new(viewport_width, 0.0, 0.0);
        let vertical = float3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - float3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner.as_vec() + u * self.horizontal + v * self.vertical
                - self.origin.as_vec(),
        }
    }
}
