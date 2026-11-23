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

/// A transparent material that refracts by its refractive index.
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    /// A transparent material with refractive index `refractive_index`.
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
        let front_face = ray.direction.dot(hit.normal) < 0.0;
        let normal = if front_face { hit.normal } else { -hit.normal };
        let ratio = if front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray.direction.unit();
        let cos_theta = (-unit_direction.dot(normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction = if ratio * sin_theta > 1.0 || schlick(cos_theta, ratio) > rng.next_f64() {
            reflect(unit_direction, normal)
        } else {
            refract(unit_direction, normal, ratio)
        };
        Some((Colour::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))
    }
}

fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    direction - normal * (2.0 * direction.dot(normal))
}

fn refract(direction: Vec3, normal: Vec3, ratio: f64) -> Vec3 {
    let cos_theta = (-direction).dot(normal).min(1.0);
    let perpendicular = (direction + normal * cos_theta) * ratio;
    let parallel = normal * -(1.0 - perpendicular.dot(perpendicular)).max(0.0).sqrt();
    perpendicular + parallel
}

fn schlick(cosine: f64, ratio: f64) -> f64 {
    let r0 = (1.0 - ratio) / (1.0 + ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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

    #[test]
    fn a_ray_entering_glass_bends_toward_the_normal() {
        let hit = HitRecord::new(
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Dielectric::new(1.5)),
        );
        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.5, 0.0, -3.0f64.sqrt() / 2.0),
        );
        let mut rng = Rng::new(1);
        let (attenuation, scattered) = Dielectric::new(1.5)
            .scatter(&ray, &hit, &mut rng)
            .expect("glass scatters");
        assert_eq!(attenuation, Vec3::new(1.0, 1.0, 1.0));
        let expected = Vec3::new(1.0 / 3.0, 0.0, -8.0f64.sqrt() / 3.0);
        assert!(
            (scattered.direction - expected).length() < 1e-9,
            "the ray refracts toward the normal: {:?}",
            scattered.direction
        );
    }

    #[test]
    fn a_ray_leaving_glass_past_the_critical_angle_reflects() {
        let hit = HitRecord::new(
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Dielectric::new(1.5)),
        );
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0));
        let mut rng = Rng::new(1);
        let (_, scattered) = Dielectric::new(1.5)
            .scatter(&ray, &hit, &mut rng)
            .expect("glass scatters");
        let expected = Vec3::new(0.5f64.sqrt(), 0.0, -0.5f64.sqrt());
        assert!(
            (scattered.direction - expected).length() < 1e-9,
            "total internal reflection sends the ray back: {:?}",
            scattered.direction
        );
    }

    #[test]
    fn grazing_incidence_reflects_where_normal_incidence_refracts() {
        let hit = HitRecord::new(
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Rc::new(Dielectric::new(1.5)),
        );
        let head_on = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rng = Rng::new(1);
        let (_, scattered) = Dielectric::new(1.5)
            .scatter(&head_on, &hit, &mut rng)
            .expect("glass scatters");
        let expected = Vec3::new(0.0, 0.0, -1.0);
        assert!(
            (scattered.direction - expected).length() < 1e-9,
            "a head-on ray refracts straight through: {:?}",
            scattered.direction
        );

        let grazing = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(85.0f64.to_radians().sin(), 0.0, -85.0f64.to_radians().cos()),
        );
        let mut rng = Rng::new(1);
        let (_, scattered) = Dielectric::new(1.5)
            .scatter(&grazing, &hit, &mut rng)
            .expect("glass scatters");
        let expected = Vec3::new(85.0f64.to_radians().sin(), 0.0, 85.0f64.to_radians().cos());
        assert!(
            (scattered.direction - expected).length() < 1e-9,
            "a grazing ray reflects: {:?}",
            scattered.direction
        );
    }
}
