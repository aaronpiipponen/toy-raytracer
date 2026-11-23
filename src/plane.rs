//! Planes.

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// An infinite plane through a point, with a unit normal.
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    /// Create a plane through `point` with `normal`.
    /// `normal` is unit length.
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
        let (u_axis, v_axis) = plane_axes(self.normal);
        let u = point.dot(u_axis).rem_euclid(1.0);
        let v = point.dot(v_axis).rem_euclid(1.0);
        Some(HitRecord::new(t, point, u, v, ray, self.normal))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}

/// Return two unit axes in the plane, perpendicular to each other and to `normal`.
fn plane_axes(normal: Vec3) -> (Vec3, Vec3) {
    // The helper is the world axis least aligned with the normal, so the two are never
    // parallel and the cross product is never zero.
    let helper = if normal.x.abs() <= normal.y.abs() && normal.x.abs() <= normal.z.abs() {
        Vec3::new(1.0, 0.0, 0.0)
    } else if normal.y.abs() <= normal.z.abs() {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    };
    let u_axis = normal.cross(helper).unit();
    (u_axis, normal.cross(u_axis))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_hits_the_plane_with_wrapped_texture_coordinates() {
        let plane = Plane::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0));
        let ray = Ray::new(Vec3::new(0.25, 0.5, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = plane.hit(&ray, 0.001, f64::INFINITY).expect("the ray hits");
        assert_eq!(hit.t, 2.0);
        assert_eq!(hit.point, Vec3::new(0.25, 0.5, -2.0));
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(hit.u, 0.5);
        assert_eq!(hit.v, 0.75);
    }
}
