//! An axis-aligned bounding box.

use crate::ray::Ray;
use crate::vec3::Vec3;

/// An axis-aligned box between `min` and `max`.
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// Return the box between the two corners `a` and `b`.
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Aabb {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        }
    }

    /// Report whether the ray meets the box with a `t` in `t_min..t_max`.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for (origin, direction, min, max) in [
            (ray.origin.x, ray.direction.x, self.min.x, self.max.x),
            (ray.origin.y, ray.direction.y, self.min.y, self.max.y),
            (ray.origin.z, ray.direction.z, self.min.z, self.max.z),
        ] {
            if direction == 0.0 {
                if origin < min || origin > max {
                    return false;
                }
                continue;
            }
            let inverse = 1.0 / direction;
            let near = (min - origin) * inverse;
            let far = (max - origin) * inverse;
            let (near, far) = if near > far { (far, near) } else { (near, far) };
            t_min = t_min.max(near);
            t_max = t_max.min(far);
            if t_max < t_min {
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
    fn a_ray_hits_the_box_it_crosses_and_misses_one_beside_it() {
        let bounds = Aabb::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, 0.5, 0.5));
        let through = Ray::new(Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(bounds.hit(&through, 0.001, f64::INFINITY));
        let beside = Ray::new(Vec3::new(2.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(!bounds.hit(&beside, 0.001, f64::INFINITY));
    }
}
