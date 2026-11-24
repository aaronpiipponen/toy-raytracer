//! Anything a ray can hit, and the record of where it hit.

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Where a ray hit a surface: the distance along the ray, the point, the normal against the ray, the
/// face side and the texture coordinates.
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
    /// Return a hit record whose normal faces against the ray, with the face side and texture
    /// coordinates.
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

    /// Return the axis-aligned box that holds every point of the object, or `None` when it has none.
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
            if object
                .bounding_box()
                .is_some_and(|bounds| !bounds.hit(ray, t_min, t_max))
            {
                continue;
            }
            let limit = nearest.map_or(t_max, |hit| hit.t);
            if let Some(hit) = object.hit(ray, t_min, limit) {
                nearest = Some(hit);
            }
        }
        nearest
    }

    /// Return none. The scene list is always tested as a whole, and each object reports its own box.
    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::Plane;
    use crate::sphere::Sphere;

    #[test]
    fn the_scene_list_still_hits_a_plane_that_has_no_box() {
        let mut world = HittableList::default();
        world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Plane::new(
            Vec3::new(0.0, -0.5, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )));
        let ray = Ray::new(Vec3::new(5.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = world
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits the plane");
        assert!((hit.t - 0.5).abs() < 1e-9);
        assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
    }
}
