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
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use rng::Rng;
use sphere::Sphere;
use vec3::{Colour, Vec3};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SAMPLES: usize = 16;
const MAX_BOUNCES: usize = 50;
const RNG_SEED: u64 = 2024;

/// The colour seen along a ray after its bounces, with the sky gradient on a miss.
fn ray_colour(ray: Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour {
    let mut attenuation = Colour::new(1.0, 1.0, 1.0);
    let mut ray = ray;
    for _ in 0..MAX_BOUNCES {
        if let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {
            match hit.material.scatter(&ray, &hit, rng) {
                Some((factor, scattered)) => {
                    attenuation = attenuation * factor;
                    ray = scattered;
                }
                None => return Colour::new(0.0, 0.0, 0.0),
            }
        } else {
            let t = 0.5 * (ray.direction.unit().y + 1.0);
            let sky = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
            return attenuation * sky;
        }
    }
    Colour::new(0.0, 0.0, 0.0)
}

fn scene() -> HittableList {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.1, -0.1, -1.0),
        0.4,
        Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.1, -0.1, -1.0),
        0.4,
        Rc::new(Dielectric::new(1.5)),
    )));
    world
}

/// Every pixel's colour averaged over `samples` jittered rays, row by row from the top.
fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut rng = Rng::new(RNG_SEED);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let mut total = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (column as f64 + rng.next_f64()) / width as f64;
                let v = (row as f64 + rng.next_f64()) / height as f64;
                total = total + ray_colour(camera.ray(u, v), &world, &mut rng);
            }
            pixels.push(total / samples as f64);
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
    let pixels = render(WIDTH, HEIGHT, SAMPLES);
    write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_center_pixel_shows_the_diffuse_sphere() {
        let pixels = render(WIDTH, HEIGHT, 1);
        let center = pixels[(HEIGHT / 2) * WIDTH + WIDTH / 2];
        assert!(
            center.x > center.y && center.x > center.z && center.x > 0.1,
            "the centre sphere is red-brown: {center:?}"
        );
    }

    #[test]
    fn the_top_row_is_sky() {
        let pixels = render(WIDTH, HEIGHT, 1);
        assert!(pixels[0].z > pixels[0].x, "the sky is bluer than it is red");
    }

    #[test]
    fn a_pixel_averages_four_jittered_samples() {
        let world = scene();
        let camera = Camera::new(1.0);
        let mut rng = Rng::new(RNG_SEED);
        let mut total = Colour::new(0.0, 0.0, 0.0);
        for _ in 0..4 {
            let u = rng.next_f64();
            let v = rng.next_f64();
            total = total + ray_colour(camera.ray(u, v), &world, &mut rng);
        }
        let expected = total / 4.0;
        let actual = render(1, 1, 4)[0];
        assert_eq!(
            actual, expected,
            "the pixel is the mean of its four jittered samples"
        );
    }

    #[test]
    fn a_mid_grey_pixel_is_written_with_gamma_two() {
        let mut out = Vec::new();
        write_ppm(&mut out, 1, 1, &[Vec3::new(0.25, 0.25, 0.25)]).expect("the image is written");
        let text = String::from_utf8(out).expect("the image is text");
        assert!(text.ends_with("127 127 127\n"), "{text:?}");
    }
}
