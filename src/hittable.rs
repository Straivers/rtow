use crate::{
    math::{Point, VectorOps},
    ray::{Hit, HitTest, Ray},
};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl HitTest for Sphere {
    fn test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin.as_vec() - self.center.as_vec();
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrt_discriminant = discriminant.sqrt();

            let mut root = (-half_b - sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrt_discriminant) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let hit_location = ray.at(root);
            let outward_normal = (hit_location.as_vec() - self.center.as_vec()) / self.radius;
            let is_front_face = ray.direction.dot(outward_normal) < 0.0;
            let normal = if is_front_face {
                outward_normal
            } else {
                -outward_normal
            };

            Some(Hit {
                point: hit_location,
                normal,
                t: root,
                is_front_face,
            })
        }
    }
}

pub enum Hittable {
    Sphere(Sphere),
}

impl HitTest for &[Hittable] {
    fn test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut hit = None;
        let mut closest = t_max;

        for hittable in self.iter() {
            match hittable {
                Hittable::Sphere(sphere) => {
                    if let Some(result) = sphere.test(ray, t_min, closest) {
                        hit = Some(result);
                        closest = result.t;
                    }
                }
            }
        }

        hit
    }
}
