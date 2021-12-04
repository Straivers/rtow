use crate::math::{float3, Point};

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

#[derive(Clone, Copy)]
pub struct Hit {
    pub point: Point,
    pub normal: float3,
    pub t: f32,
    pub is_front_face: bool,
}

pub trait HitTest {
    fn test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
