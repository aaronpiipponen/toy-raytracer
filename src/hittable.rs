//! Anything a ray can hit, and the record of where it hit.

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Where a ray hit a surface: the distance along the ray, the point, the normal that always faces
/// against the ray, the face side and the texture coordinates.
#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f64,
    // The origin of the bounce rays.
    #[allow(dead_code)]
    pub point: Vec3,
    pub normal: Vec3,
    #[allow(dead_code)]
    pub front_face: bool,
    #[allow(dead_code)]
    pub u: f64,
    #[allow(dead_code)]
    pub v: f64,
}

impl HitRecord {
    /// Build a hit record whose normal faces against the ray, with the face side recorded.
    pub fn new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3, u: f64, v: f64) -> Self {
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

    /// The axis-aligned box around the whole object, or `None` when the object has no finite
    /// extent.
    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
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
            let limit = nearest.map_or(t_max, |hit| hit.t);
            if let Some(bounds) = object.bounding_box() {
                if !bounds.hit(ray, t_min, limit) {
                    continue;
                }
            }
            if let Some(hit) = object.hit(ray, t_min, limit) {
                nearest = Some(hit);
            }
        }
        nearest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::Plane;

    struct Unreachable;

    impl Hittable for Unreachable {
        fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
            panic!("the list tests an object whose box the ray misses");
        }

        fn bounding_box(&self) -> Option<Aabb> {
            Some(Aabb::new(
                Vec3::new(10.0, 10.0, -6.0),
                Vec3::new(11.0, 11.0, -5.0),
            ))
        }
    }

    #[test]
    fn the_list_tests_an_object_without_a_box_and_skips_one_whose_box_the_ray_misses() {
        let mut world = HittableList::default();
        world.add(Box::new(Unreachable));
        world.add(Box::new(Plane::new(
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
        )));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = world
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the plane is hit");
        assert!((hit.t - 1.0).abs() < 1e-9);
    }
}
