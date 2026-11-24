//! Materials: how a ray scatters when it hits a surface.

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::vec3::{Colour, Vec3};

/// A surface material: how an incoming ray scatters, or nothing when it is absorbed.
pub trait Material {
    /// The attenuation and the scattered ray, or nothing when the ray is absorbed.
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>;
}

/// A diffuse material that scatters in a random direction around the normal.
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    /// A diffuse material with the given albedo.
    pub fn new(albedo: Colour) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
        let direction = hit.normal + random_unit_vector(rng);
        Some((self.albedo, Ray::new(hit.point, direction)))
    }
}

/// A metal material that reflects about the normal, blurred by a fuzz factor in 0..=1.
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
    /// A metal with the given albedo and a fuzz factor in 0..=1.
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
        let reflected = reflect(ray_in.direction.unit(), hit.normal);
        let scattered = reflected + random_unit_vector(rng) * self.fuzz;
        if scattered.dot(hit.normal) <= 0.0 {
            None
        } else {
            Some((self.albedo, Ray::new(hit.point, scattered)))
        }
    }
}

/// The mirror reflection of `v` about the normal `n`.
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

/// A random direction on the unit sphere, drawn by rejection sampling a point in the unit cube.
fn random_unit_vector(rng: &mut Rng) -> Vec3 {
    loop {
        let p = Vec3::new(
            2.0 * rng.next_f64() - 1.0,
            2.0 * rng.next_f64() - 1.0,
            2.0 * rng.next_f64() - 1.0,
        );
        let len_squared = p.dot(p);
        if len_squared > 1e-12 && len_squared <= 1.0 {
            return p.unit();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    fn record_with(normal: Vec3, material: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            t: 1.0,
            point: Vec3::new(0.0, 0.0, -1.0),
            normal,
            material,
        }
    }

    #[test]
    fn a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface() {
        let albedo = Colour::new(0.7, 0.3, 0.3);
        let hit = record_with(Vec3::new(0.0, 0.0, 1.0), Rc::new(Lambertian::new(albedo)));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rng = Rng::new(7);
        for _ in 0..3 {
            let (attenuation, scattered) = hit
                .material
                .scatter(&ray, &hit, &mut rng)
                .expect("the diffuse material scatters");
            assert_eq!(attenuation, albedo);
            assert!(scattered.direction.dot(hit.normal) > 0.0);
        }
    }

    #[test]
    fn a_metal_with_no_fuzz_reflects_about_the_normal() {
        let albedo = Colour::new(0.8, 0.8, 0.8);
        let hit = record_with(Vec3::new(0.0, 0.0, 1.0), Rc::new(Metal::new(albedo, 0.0)));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rng = Rng::new(7);
        let (attenuation, scattered) = hit
            .material
            .scatter(&ray, &hit, &mut rng)
            .expect("the metal scatters");
        assert_eq!(attenuation, albedo);
        let direction = scattered.direction;
        assert!((direction.x - 0.0).abs() < 1e-9);
        assert!((direction.y - 0.0).abs() < 1e-9);
        assert!((direction.z - 1.0).abs() < 1e-9);
    }

    #[test]
    fn a_metal_reflection_into_the_surface_is_absorbed() {
        let albedo = Colour::new(0.8, 0.8, 0.8);
        let hit = record_with(Vec3::new(0.0, 0.0, -1.0), Rc::new(Metal::new(albedo, 0.0)));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rng = Rng::new(7);
        assert!(hit.material.scatter(&ray, &hit, &mut rng).is_none());
    }
}
