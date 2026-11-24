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

/// A glass that refracts by its refractive index and reflects by total internal reflection or
/// Schlick's approximation.
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    /// A glass with the given refractive index.
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
        let unit_direction = ray_in.direction.unit();
        let eta_ratio = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let cos_theta = (-unit_direction.dot(hit.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction =
            if eta_ratio * sin_theta > 1.0 || rng.next_f64() < schlick(cos_theta, eta_ratio) {
                reflect(unit_direction, hit.normal)
            } else {
                refract(unit_direction, hit.normal, eta_ratio)
            };
        Some((Colour::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))
    }
}

/// The mirror reflection of `v` about the normal `n`.
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

/// The refracted direction of a unit ray `v` crossing a surface with unit normal `n`, for the
/// ratio `eta_ratio` of the index the ray leaves to the index it enters.
fn refract(v: Vec3, n: Vec3, eta_ratio: f64) -> Vec3 {
    let cos_theta = (-v.dot(n)).min(1.0);
    let perpendicular = (v + n * cos_theta) * eta_ratio;
    let parallel = n * -(1.0 - perpendicular.dot(perpendicular)).max(0.0).sqrt();
    perpendicular + parallel
}

/// Schlick's approximation of the reflectance at `cosine` for index ratio `eta_ratio`.
fn schlick(cosine: f64, eta_ratio: f64) -> f64 {
    let r0 = (1.0 - eta_ratio) / (1.0 + eta_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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

    fn record_with(ray: &Ray, outward_normal: Vec3, material: Rc<dyn Material>) -> HitRecord {
        HitRecord::new(ray, 1.0, ray.at(1.0), outward_normal, 0.0, 0.0, material)
    }

    #[test]
    fn a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface() {
        let albedo = Colour::new(0.7, 0.3, 0.3);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = record_with(
            &ray,
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Lambertian::new(albedo)),
        );
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
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = record_with(
            &ray,
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Metal::new(albedo, 0.0)),
        );
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
    fn a_fuzzy_metal_reflection_into_the_surface_is_absorbed() {
        let albedo = Colour::new(0.8, 0.8, 0.8);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, -0.05));
        let hit = record_with(
            &ray,
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Metal::new(albedo, 1.0)),
        );
        let mut rng = Rng::new(7);
        let mut absorbed = false;
        for _ in 0..50 {
            if hit.material.scatter(&ray, &hit, &mut rng).is_none() {
                absorbed = true;
                break;
            }
        }
        assert!(
            absorbed,
            "the fuzzy metal absorbs a reflection into the surface"
        );
    }

    #[test]
    fn a_glass_ray_inside_reflects_on_total_internal_reflection() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.8, 0.0, 0.6));
        let hit = record_with(
            &ray,
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Dielectric::new(1.5)),
        );
        assert!(!hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        let mut rng = Rng::new(7);
        let (attenuation, scattered) = hit
            .material
            .scatter(&ray, &hit, &mut rng)
            .expect("glass never absorbs");
        assert_eq!(attenuation, Colour::new(1.0, 1.0, 1.0));
        let direction = scattered.direction;
        assert!((direction.x - 0.8).abs() < 1e-9);
        assert!((direction.y - 0.0).abs() < 1e-9);
        assert!((direction.z + 0.6).abs() < 1e-9);
    }

    #[test]
    fn a_glass_ray_bends_by_snell_when_it_enters() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.6, 0.0, -0.8));
        let hit = record_with(
            &ray,
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Dielectric::new(1.5)),
        );
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        let mut rng = Rng::new(7);
        let (attenuation, scattered) = hit
            .material
            .scatter(&ray, &hit, &mut rng)
            .expect("glass never absorbs");
        assert_eq!(attenuation, Colour::new(1.0, 1.0, 1.0));
        let direction = scattered.direction;
        assert!((direction.x - 0.4).abs() < 1e-9);
        assert!((direction.y - 0.0).abs() < 1e-9);
        assert!((direction.z + 0.916_515_139_199_514_4).abs() < 1e-9);
    }
}
