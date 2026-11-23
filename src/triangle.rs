//! Triangles.

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Triangle { a, b, c }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let pvec = ray.direction.cross(edge2);
        let determinant = edge1.dot(pvec);
        // A determinant near zero means the ray runs parallel to the triangle's plane.
        if determinant.abs() < 1e-8 {
            return None;
        }
        let inverse = 1.0 / determinant;
        let tvec = ray.origin - self.a;
        let u = tvec.dot(pvec) * inverse;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let qvec = tvec.cross(edge1);
        let v = ray.direction.dot(qvec) * inverse;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = edge2.dot(qvec) * inverse;
        if t <= t_min || t >= t_max {
            return None;
        }
        let point = ray.at(t);
        let outward_normal = edge1.cross(edge2).unit();
        Some(HitRecord::new(t, point, u, v, ray, outward_normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_hits_the_triangle_with_barycentric_texture_coordinates() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(1.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0, -2.0),
        );
        let ray = Ray::new(Vec3::new(0.25, 0.25, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = triangle
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits");
        assert_eq!(hit.t, 2.0);
        assert_eq!(hit.point, Vec3::new(0.25, 0.25, -2.0));
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(hit.u, 0.25);
        assert_eq!(hit.v, 0.25);
    }

    #[test]
    fn a_ray_beside_the_triangle_misses() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(1.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0, -2.0),
        );
        let ray = Ray::new(Vec3::new(2.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(triangle.hit(&ray, 0.001, f64::INFINITY).is_none());
    }
}
