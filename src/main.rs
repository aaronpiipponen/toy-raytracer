//! Render a fixed scene and write it to standard output as a PPM image.

mod camera;
mod hittable;
mod material;
mod ray;
mod rng;
mod sphere;
mod vec3;

use std::io::{self, Write};
use std::rc::Rc;

use camera::Camera;
use hittable::{Hittable, HittableList};
use material::{Lambertian, Metal};
use ray::Ray;
use rng::Rng;
use sphere::Sphere;
use vec3::{Colour, Vec3};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SEED: u64 = 7;
const MAX_BOUNCES: usize = 50;

/// The colour a ray sees: its scattered path attenuated by each material, the sky gradient when it
/// misses.
fn ray_colour(ray: &Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour {
    let mut ray = *ray;
    let mut colour = Vec3::new(1.0, 1.0, 1.0);
    for _ in 0..MAX_BOUNCES {
        if let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {
            match hit.material.scatter(&ray, &hit, rng) {
                Some((attenuation, scattered)) => {
                    colour = colour * attenuation;
                    ray = scattered;
                }
                None => return Vec3::new(0.0, 0.0, 0.0),
            }
        } else {
            return colour * sky_colour(&ray);
        }
    }
    Vec3::new(0.0, 0.0, 0.0)
}

/// Return the sky gradient for a ray that hits nothing.
fn sky_colour(ray: &Ray) -> Colour {
    let t = 0.5 * (ray.direction.unit().y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn scene() -> HittableList {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.85, -0.25, -1.3),
        0.25,
        Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3)),
    )));
    world
}

/// Every pixel's colour, row by row from the top.
fn render(width: usize, height: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut rng = Rng::new(SEED);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let u = column as f64 / (width - 1) as f64;
            let v = row as f64 / (height - 1) as f64;
            pixels.push(ray_colour(&camera.ray(u, v), &world, &mut rng));
        }
    }
    pixels
}

fn to_byte(channel: f64) -> u8 {
    (255.999 * channel.clamp(0.0, 1.0).sqrt()) as u8
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
    fn the_top_row_is_sky() {
        let pixels = render(WIDTH, HEIGHT);
        assert!(pixels[0].z > pixels[0].x, "the sky is bluer than it is red");
    }
}
