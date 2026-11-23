//! Spheres.

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let root = discriminant.sqrt();
        let mut t = (-half_b - root) / a;
        if t <= t_min || t >= t_max {
            t = (-half_b + root) / a;
            if t <= t_min || t >= t_max {
                return None;
            }
        }
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let u = 0.5 + outward_normal.z.atan2(outward_normal.x) / (2.0 * PI);
        let v = 0.5 - outward_normal.y.asin() / PI;
        Some(HitRecord::new(t, point, u, v, ray, outward_normal))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        Some(Aabb::new(self.center - radius, self.center + radius))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_through_the_center_hits_the_near_surface() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = sphere
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits");
        assert!((hit.t - 1.5).abs() < 1e-9);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert!(hit.front_face);
        assert!((hit.u - 0.75).abs() < 1e-9);
        assert!((hit.v - 0.5).abs() < 1e-9);
    }

    #[test]
    fn a_ray_from_inside_the_sphere_flips_the_normal() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits");
        assert!(!hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn a_ray_beside_the_sphere_misses() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5);
        let ray = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(sphere.hit(&ray, 0.001, f64::INFINITY).is_none());
    }
}
