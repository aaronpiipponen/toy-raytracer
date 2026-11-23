//! Anything a ray can hit, and the record of where it hit.

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Where a ray hit a surface: the distance along the ray, the point, the normal against the
/// ray, the face side and the texture coordinates.
#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f64,
    #[allow(dead_code)]
    pub point: Vec3,
    pub normal: Vec3,
    // Read by the tests only.
    #[allow(dead_code)]
    pub front_face: bool,
    #[allow(dead_code)]
    pub u: f64,
    #[allow(dead_code)]
    pub v: f64,
}

impl HitRecord {
    /// Return a hit at distance `t` and `point`, with texture coordinates `(u, v)` and a normal
    /// facing `ray`. `outward_normal` is unit length and points away from the surface.
    pub fn new(t: f64, point: Vec3, u: f64, v: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            t,
            point,
            normal,
            front_face,
            u,
            v,
        }
    }
}

pub trait Hittable {
    /// The nearest hit with `t` in `t_min..t_max`, if any.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// The box used to skip objects the ray misses, or `None` when every ray must test the object.
    fn bounding_box(&self) -> Option<Aabb>;
}

/// A scene: every object, hit in order of distance.
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut nearest: Option<HitRecord> = None;
        for object in &self.objects {
            let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);
            if let Some(bounding_box) = object.bounding_box() {
                if !bounding_box.hit(ray, t_min, limit) {
                    continue;
                }
            }
            if let Some(hit) = object.hit(ray, t_min, limit) {
                nearest = Some(hit);
            }
        }
        nearest
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// A shape that counts its tests and never reports a hit.
    struct Probe {
        bounding_box: Aabb,
        calls: Rc<Cell<usize>>,
    }

    impl Hittable for Probe {
        fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
            self.calls.set(self.calls.get() + 1);
            None
        }

        fn bounding_box(&self) -> Option<Aabb> {
            Some(self.bounding_box)
        }
    }

    #[test]
    fn the_list_skips_an_object_when_the_ray_misses_its_box() {
        let calls = Rc::new(Cell::new(0));
        let mut world = HittableList::default();
        world.add(Box::new(Probe {
            bounding_box: Aabb::new(Vec3::new(5.0, 5.0, -3.0), Vec3::new(6.0, 6.0, -2.0)),
            calls: Rc::clone(&calls),
        }));
        let beside = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(world.hit(&beside, 0.001, f64::INFINITY).is_none());
        assert_eq!(calls.get(), 0);
        let through = Ray::new(Vec3::new(5.5, 5.5, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(world.hit(&through, 0.001, f64::INFINITY).is_none());
        assert_eq!(calls.get(), 1);
    }
}
