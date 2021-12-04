use crate::math::{Point, float3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: float3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}