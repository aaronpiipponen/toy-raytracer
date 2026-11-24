//! An infinite plane.

use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// An infinite plane through `point` with the unit `normal` and the given material.
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

impl Plane {
    /// Return a plane through `point` with `normal` scaled to unit length and the given material.
    pub fn new(point: Vec3, normal: Vec3, material: Rc<dyn Material>) -> Self {
        Plane {
            point,
            normal: normal.unit(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denominator = self.normal.dot(ray.direction);
        if denominator.abs() < 1e-9 {
            return None;
        }
        let t = (self.point - ray.origin).dot(self.normal) / denominator;
        if t <= t_min || t >= t_max {
            return None;
        }
        let point = ray.at(t);
        let (u, v) = if self.normal.x.abs() >= self.normal.y.abs()
            && self.normal.x.abs() >= self.normal.z.abs()
        {
            (point.y, point.z)
        } else if self.normal.y.abs() >= self.normal.z.abs() {
            (point.x, point.z)
        } else {
            (point.x, point.y)
        };
        Some(HitRecord::new(
            ray,
            t,
            point,
            self.normal,
            u.rem_euclid(1.0),
            v.rem_euclid(1.0),
            Rc::clone(&self.material),
        ))
    }

    /// Return none: an infinite plane has no bounding box.
    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;
    use crate::vec3::Colour;

    #[test]
    fn a_ray_hits_the_plane_and_its_texture_coordinates_wrap() {
        let plane = Plane::new(
            Vec3::new(0.0, -0.5, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))),
        );
        let ray = Ray::new(Vec3::new(1.25, 0.0, -0.25), Vec3::new(0.0, -1.0, 0.0));
        let hit = plane.hit(&ray, 0.001, f64::INFINITY).expect("the ray hits");
        assert!((hit.t - 0.5).abs() < 1e-9);
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
        assert!((hit.u - 0.25).abs() < 1e-9);
        assert!((hit.v - 0.75).abs() < 1e-9);
    }
}
