//! Render a fixed scene and write it to standard output as a PPM image.

mod camera;
mod hittable;
#[allow(dead_code)]
mod plane;
mod ray;
#[allow(dead_code)]
mod rng;
mod sphere;
mod triangle;
mod vec3;

use std::io::{self, Write};

use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use triangle::Triangle;
use vec3::{Colour, Vec3};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

/// The colour seen along a ray: the surface normal where it hits, the sky gradient otherwise.
fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        return (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let t = 0.5 * (ray.direction.unit().y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn scene() -> HittableList {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Box::new(Triangle::new(
        Vec3::new(0.6, 0.15, -1.0),
        Vec3::new(1.95, 0.15, -1.0),
        Vec3::new(1.275, 0.92, -1.0),
    )));
    world
}

/// Every pixel's colour, row by row from the top.
fn render(width: usize, height: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let u = column as f64 / (width - 1) as f64;
            let v = row as f64 / (height - 1) as f64;
            pixels.push(ray_colour(&camera.ray(u, v), &world));
        }
    }
    pixels
}

fn to_byte(channel: f64) -> u8 {
    (255.999 * channel.clamp(0.0, 1.0)) as u8
}

fn write_ppm(
    out: &mut impl Write,
    width: usize,
    height: usize,
    pixels: &[Colour],
) -> io::Result<()> {
    writeln!(out, "P3\n{width} {height}\n255")?;
    for pixel in pixels {
        writeln!(
            out,
            "{} {} {}",
            to_byte(pixel.x),
            to_byte(pixel.y),
            to_byte(pixel.z)
        )?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let pixels = render(WIDTH, HEIGHT);
    write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_center_pixel_shows_the_sphere_facing_the_camera() {
        let pixels = render(WIDTH, HEIGHT);
        let center = pixels[(HEIGHT / 2) * WIDTH + WIDTH / 2];
        assert!(center.z > 0.9, "the normal faces the camera: {center:?}");
    }

    #[test]
    fn the_top_row_is_sky() {
        let pixels = render(WIDTH, HEIGHT);
        assert!(pixels[0].z > pixels[0].x, "the sky is bluer than it is red");
    }
}
