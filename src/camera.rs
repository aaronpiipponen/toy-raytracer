//! A pinhole camera at the origin looking down the negative z axis.

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// A camera whose viewport has the image's aspect ratio and a height of 2.
    pub fn new(aspect_ratio: f64) -> Self {
        let height = 2.0;
        let width = aspect_ratio * height;
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, height, 0.0);
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, 1.0);
        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    /// The ray through the viewport point (u, v), both in 0..=1 from the lower left.
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
