//! A triangle.

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A triangle with the vertices `a`, `b` and `c` in counter-clockwise order seen from outside.
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    /// Return a triangle through the three vertices.
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Triangle { a, b, c }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.direction.cross(edge2);
        let determinant = edge1.dot(h);
        if determinant.abs() < 1e-8 {
            return None;
        }
        let inverse_determinant = 1.0 / determinant;
        let s = ray.origin - self.a;
        let u = inverse_determinant * s.dot(h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(edge1);
        let v = inverse_determinant * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = inverse_determinant * edge2.dot(q);
        if t <= t_min || t >= t_max {
            return None;
        }
        let outward_normal = edge1.cross(edge2).unit();
        Some(HitRecord::new(ray, t, ray.at(t), outward_normal, u, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_through_the_triangle_hits_it_with_barycentric_coordinates() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(1.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0, -2.0),
        );
        let ray = Ray::new(Vec3::new(0.25, 0.25, -1.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = triangle
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits");
        assert!((hit.t - 1.0).abs() < 1e-9);
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert!((hit.u - 0.25).abs() < 1e-9);
        assert!((hit.v - 0.25).abs() < 1e-9);
    }

    #[test]
    fn a_ray_beside_the_triangle_misses() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(1.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0, -2.0),
        );
        let ray = Ray::new(Vec3::new(2.0, 2.0, -1.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(triangle.hit(&ray, 0.001, f64::INFINITY).is_none());
    }
}
