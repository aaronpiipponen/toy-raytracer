//! Materials: how a ray scatters off a surface.

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::vec3::{Colour, Vec3};

/// How a ray scatters off a surface.
pub trait Material {
    /// Return the attenuation and the scattered ray, or `None` when the ray is absorbed.
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>;
}

/// A diffuse surface that scatters in a random direction around the normal.
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    /// A diffuse material that attenuates by `albedo`.
    pub fn new(albedo: Colour) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
        Some((
            self.albedo,
            Ray::new(hit.point, hit.normal + random_unit_vector(rng)),
        ))
    }
}

/// A metal surface that reflects about the normal, blurred by `fuzz`.
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
    /// A metal material that attenuates by `albedo` and blurs its reflection by `fuzz` in `0.0..=1.0`.
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
        let reflected = reflect(ray.direction.unit(), hit.normal);
        let scattered = Ray::new(hit.point, reflected + random_unit_vector(rng) * self.fuzz);
        if scattered.direction.dot(hit.normal) <= 0.0 {
            return None;
        }
        Some((self.albedo, scattered))
    }
}

fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    direction - normal * (2.0 * direction.dot(normal))
}

fn random_unit_vector(rng: &mut Rng) -> Vec3 {
    loop {
        let candidate = Vec3::new(
            2.0 * rng.next_f64() - 1.0,
            2.0 * rng.next_f64() - 1.0,
            2.0 * rng.next_f64() - 1.0,
        );
        if candidate.length() < 1.0 {
            return candidate.unit();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn a_diffuse_scatter_attenuates_by_its_albedo_and_stays_above_the_surface() {
        let albedo = Vec3::new(0.7, 0.2, 0.1);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let hit = HitRecord::new(
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            normal,
            Rc::new(Lambertian::new(albedo)),
        );
        let ray = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rng = Rng::new(7);
        let (attenuation, scattered) = Lambertian::new(albedo)
            .scatter(&ray, &hit, &mut rng)
            .expect("a diffuse material scatters");
        assert_eq!(attenuation, albedo);
        assert!(scattered.direction.dot(normal) > 0.0);
    }

    #[test]
    fn metal_absorbs_a_reflection_that_points_into_the_surface() {
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let hit = HitRecord::new(
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            normal,
            Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
        );
        let ray = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let mut rng = Rng::new(7);
        assert!(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.0)
            .scatter(&ray, &hit, &mut rng)
            .is_none());
    }
}
