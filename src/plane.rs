//! Planes.

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// An infinite plane: a point on it and its unit normal.
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    /// Build a plane through `point` with unit normal `normal`.
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Plane { point, normal }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denominator = ray.direction.dot(self.normal);
        if denominator == 0.0 {
            return None;
        }
        let t = (self.point - ray.origin).dot(self.normal) / denominator;
        if t <= t_min || t >= t_max {
            return None;
        }
        let point = ray.at(t);
        let tangent = if self.normal.x.abs() > self.normal.y.abs() {
            Vec3::new(-self.normal.z, 0.0, self.normal.x).unit()
        } else {
            Vec3::new(0.0, -self.normal.z, self.normal.y).unit()
        };
        let bitangent = self.normal.cross(tangent);
        let u = point.dot(tangent).rem_euclid(1.0);
        let v = point.dot(bitangent).rem_euclid(1.0);
        Some(HitRecord::new(ray, t, point, self.normal, u, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_through_the_plane_records_the_hit_and_texture_coordinates_that_wrap() {
        let plane = Plane::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0));
        let ray = Ray::new(Vec3::new(2.25, -0.5, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = plane.hit(&ray, 0.001, f64::INFINITY).expect("the ray hits");
        assert!((hit.t - 2.0).abs() < 1e-9);
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert!((hit.u - 0.5).abs() < 1e-9);
        assert!((hit.v - 0.25).abs() < 1e-9);
    }
}
