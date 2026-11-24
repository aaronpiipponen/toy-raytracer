//! Materials: how a ray scatters when it hits a surface.

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::vec3::{Colour, Vec3};

/// A surface material: how an incoming ray scatters, or nothing when it is absorbed.
pub trait Material {
    /// Scatter an incoming ray, returning the attenuation and the scattered ray, or `None` when the
    /// surface absorbs the ray.
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>;
}

/// A matte surface that scatters in a random direction around the normal.
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    /// Create a matte material with the given albedo.
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
    /// Create a metal material with the given albedo and fuzz factor in 0..=1.
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

/// A transparent surface that refracts by its refractive index.
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    /// Create a dielectric material with the given refractive index.
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction.unit();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || schlick_reflectance(cos_theta, refraction_ratio) > rng.next_f64()
        {
            reflect(unit_direction, hit.normal)
        } else {
            refract(unit_direction, hit.normal, refraction_ratio)
        };
        Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

fn refract(unit_direction: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3 {
    let cos_theta = (-unit_direction).dot(normal).min(1.0);
    let perpendicular = (unit_direction + normal * cos_theta) * refraction_ratio;
    let parallel = normal * -(1.0 - perpendicular.dot(perpendicular)).abs().sqrt();
    perpendicular + parallel
}

/// Return Schlick's reflection share at `cos_theta` for an index ratio of `refraction_ratio`.
fn schlick_reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

/// Return a random direction on the unit sphere, drawn from `rng` by rejection sampling.
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

    /// Return a hit at the point `(0, 0, -1)` whose facing normal is `(0, 0, 1)`.
    fn facing_hit(material: &Rc<dyn Material>, ray: &Ray) -> HitRecord {
        HitRecord::new(
            ray,
            1.0,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
            0.0,
            0.0,
            Rc::clone(material),
        )
    }

    #[test]
    fn a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface() {
        let mut rng = Rng::new(7);
        let material: Rc<dyn Material> = Rc::new(Lambertian::new(Colour::new(0.5, 0.6, 0.7)));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = facing_hit(&material, &ray);
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
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = facing_hit(&material, &ray);
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
        let material: Rc<dyn Material> = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 1.0));
        // A ray almost parallel to the surface reflects almost along it, so the fuzz alone can
        // push the scattered direction into the surface.
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, -0.01));
        let hit = facing_hit(&material, &ray);
        let absorbed = (0..100).any(|_| material.scatter(&ray, &hit, &mut rng).is_none());
        assert!(
            absorbed,
            "a fuzzed reflection eventually points into the surface"
        );
    }

    #[test]
    fn a_glass_ray_at_normal_incidence_passes_through_the_surface() {
        let mut rng = Rng::new(7);
        let material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = facing_hit(&material, &ray);
        let (attenuation, scattered) = material
            .scatter(&ray, &hit, &mut rng)
            .expect("a glass ray at normal incidence refracts");
        assert_eq!(attenuation, Vec3::new(1.0, 1.0, 1.0));
        assert!((scattered.direction.x - 0.0).abs() < 1e-9);
        assert!((scattered.direction.y - 0.0).abs() < 1e-9);
        assert!((scattered.direction.z + 1.0).abs() < 1e-9);
    }

    #[test]
    fn a_glass_ray_entering_at_an_angle_bends_toward_the_normal() {
        let mut rng = Rng::new(7);
        let material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.8, 0.0, -0.6));
        let hit = facing_hit(&material, &ray);
        let (attenuation, scattered) = material
            .scatter(&ray, &hit, &mut rng)
            .expect("a glass ray entering at an angle refracts");
        assert_eq!(attenuation, Vec3::new(1.0, 1.0, 1.0));
        let x: f64 = 0.8 * (2.0 / 3.0);
        let z = -(1.0 - x * x).sqrt();
        assert!((scattered.direction.x - x).abs() < 1e-9);
        assert!((scattered.direction.y - 0.0).abs() < 1e-9);
        assert!((scattered.direction.z - z).abs() < 1e-9);
    }

    #[test]
    fn a_glass_ray_leaving_beyond_the_critical_angle_reflects_internally() {
        let mut rng = Rng::new(7);
        let material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.8, 0.0, 0.6));
        let hit = facing_hit(&material, &ray);
        let (attenuation, scattered) = material
            .scatter(&ray, &hit, &mut rng)
            .expect("a glass ray beyond the critical angle reflects");
        assert_eq!(attenuation, Vec3::new(1.0, 1.0, 1.0));
        assert!((scattered.direction.x - 0.8).abs() < 1e-9);
        assert!((scattered.direction.y - 0.0).abs() < 1e-9);
        assert!((scattered.direction.z + 0.6).abs() < 1e-9);
    }
}
