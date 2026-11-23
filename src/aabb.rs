//! Axis-aligned bounding boxes.

use crate::ray::Ray;
use crate::vec3::Vec3;

/// An axis-aligned box whose sides lie along the coordinate axes.
#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// Create the box whose corners are `min` and `max`.
    /// `min` is the lower corner in every axis.
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb { min, max }
    }

    /// Return whether the ray meets the box with `t` in `t_min..=t_max`.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_near = t_min;
        let mut t_far = t_max;
        let slabs = [
            (self.min.x, self.max.x, ray.origin.x, ray.direction.x),
            (self.min.y, self.max.y, ray.origin.y, ray.direction.y),
            (self.min.z, self.max.z, ray.origin.z, ray.direction.z),
        ];
        for (min, max, origin, direction) in slabs {
            let inverse = 1.0 / direction;
            // A zero `direction` gives infinite bounds, and a bound that is NaN because the
            // origin lies exactly on that face is ignored by `max` and `min`.
            let mut t0 = (min - origin) * inverse;
            let mut t1 = (max - origin) * inverse;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_near = t_near.max(t0);
            t_far = t_far.min(t1);
            if t_near > t_far {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_through_the_box_hits_it_and_a_ray_beside_it_misses() {
        let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let through = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(aabb.hit(&through, 0.001, f64::INFINITY));
        let beside = Ray::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(!aabb.hit(&beside, 0.001, f64::INFINITY));
    }
}
