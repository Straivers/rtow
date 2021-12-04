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

    pub fn hit_sphere(&self, circle_center: &Point, radius: f32) -> bool {
        let oc = self.origin.as_vec() - circle_center.as_vec();
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        return discriminant > 0.0;
    }
}
