//! Materials: how a ray scatters when it hits a surface.

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::vec3::{Colour, Vec3};

/// A surface material: how an incoming ray scatters, or nothing when it is absorbed.
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>;
}

/// A matte surface that scatters in a random direction around the normal.
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
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

/// A reflective surface whose fuzz factor is in 0..=1.
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
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

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

/// A random direction on the unit sphere, drawn from `rng` by rejection sampling.
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

    #[test]
    fn a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface() {
        let mut rng = Rng::new(7);
        let material: Rc<dyn Material> = Rc::new(Lambertian::new(Colour::new(0.5, 0.6, 0.7)));
        let hit = HitRecord {
            t: 1.0,
            point: Vec3::new(0.0, 0.0, -1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: Rc::clone(&material),
        };
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        for _ in 0..3 {
            let (attenuation, scattered) = material
                .scatter(&ray, &hit, &mut rng)
                .expect("a diffuse material always scatters");
            assert_eq!(attenuation, Vec3::new(0.5, 0.6, 0.7));
            assert!(scattered.direction.dot(hit.normal) > 0.0);
        }
    }

    #[test]
    fn a_metal_with_no_fuzz_reflects_about_the_normal() {
        let mut rng = Rng::new(7);
        let material: Rc<dyn Material> = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.0));
        let hit = HitRecord {
            t: 1.0,
            point: Vec3::new(0.0, 0.0, -1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: Rc::clone(&material),
        };
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let (attenuation, scattered) = material
            .scatter(&ray, &hit, &mut rng)
            .expect("a reflection away from the surface scatters");
        assert_eq!(attenuation, Vec3::new(0.8, 0.8, 0.8));
        assert!((scattered.direction.x - 0.0).abs() < 1e-9);
        assert!((scattered.direction.y - 0.0).abs() < 1e-9);
        assert!((scattered.direction.z - 1.0).abs() < 1e-9);
    }

    #[test]
    fn a_metal_reflection_into_the_surface_is_absorbed() {
        let mut rng = Rng::new(7);
        let material: Rc<dyn Material> = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.0));
        let hit = HitRecord {
            t: 1.0,
            point: Vec3::new(0.0, 0.0, -1.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
            material: Rc::clone(&material),
        };
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(material.scatter(&ray, &hit, &mut rng).is_none());
    }
}
