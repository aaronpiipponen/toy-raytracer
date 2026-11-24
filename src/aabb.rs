//! Axis-aligned bounding boxes.

use crate::ray::Ray;
use crate::vec3::Vec3;

/// The box between two opposite corners.
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// Build a box from opposite corners, `min` low on every axis.
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb { min, max }
    }

    /// Report whether the ray crosses the box with some `t` in `t_min..t_max`.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let origin = [ray.origin.x, ray.origin.y, ray.origin.z];
        let direction = [ray.direction.x, ray.direction.y, ray.direction.z];
        let min = [self.min.x, self.min.y, self.min.z];
        let max = [self.max.x, self.max.y, self.max.z];
        let mut t_enter = t_min;
        let mut t_exit = t_max;
        for axis in 0..3 {
            if direction[axis] == 0.0 {
                if origin[axis] < min[axis] || origin[axis] > max[axis] {
                    return false;
                }
                continue;
            }
            let inverse = 1.0 / direction[axis];
            let mut near = (min[axis] - origin[axis]) * inverse;
            let mut far = (max[axis] - origin[axis]) * inverse;
            if near > far {
                (near, far) = (far, near);
            }
            t_enter = t_enter.max(near);
            t_exit = t_exit.min(far);
            if t_enter > t_exit {
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
    fn a_ray_hits_the_box_it_crosses_and_misses_the_box_beside_it() {
        let bounds = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let through = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let beside = Ray::new(Vec3::new(2.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(bounds.hit(&through, 0.001, f64::INFINITY));
        assert!(!bounds.hit(&beside, 0.001, f64::INFINITY));
    }
}
