//! Render a fixed scene and write it to a file as a PNG or PPM image.

mod aabb;
mod camera;
mod hittable;
mod material;
#[allow(dead_code)]
mod plane;
mod png;
mod ray;
mod rng;
mod sphere;
mod triangle;
mod vec3;

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use camera::Camera;
use hittable::{Hittable, HittableList};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use rng::Rng;
use sphere::Sphere;
use triangle::Triangle;
use vec3::{Colour, Vec3};

const DEFAULT_WIDTH: usize = 400;
const DEFAULT_HEIGHT: usize = 200;
const DEFAULT_SAMPLES: usize = 16;
const DEFAULT_OUTPUT: &str = "render.png";
const SEED: u64 = 7;
const MAX_BOUNCES: usize = 50;

const USAGE: &str = "usage: toy-raytracer [--width <pixels>] [--height <pixels>] [--samples <count>] [--output <path>] [--format png|ppm]";

#[derive(Clone, Copy)]
enum Format {
    Png,
    Ppm,
}

struct Settings {
    width: usize,
    height: usize,
    samples: usize,
    output: PathBuf,
    format: Format,
}

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

/// The sky gradient a ray sees when it misses the scene: white at the horizon, blue above.
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
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.1, 0.0, -1.5),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Triangle::new(
        Vec3::new(0.6, 0.15, -1.0),
        Vec3::new(1.95, 0.15, -1.0),
        Vec3::new(1.275, 0.92, -1.0),
        Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3))),
    )));
    world
}

/// Every pixel's colour, row by row from the top, each pixel averaging `samples` jittered rays, with
/// the share of finished rows reported on standard error.
fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut rng = Rng::new(SEED);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (column as f64 + rng.next_f64()) / width as f64;
                let v = (row as f64 + rng.next_f64()) / height as f64;
                pixel = pixel + ray_colour(&camera.ray(u, v), &world, &mut rng);
            }
            pixels.push(pixel / samples as f64);
        }
        let done = height - row;
        eprint!("\r{}%", done * 100 / height);
    }
    eprintln!();
    pixels
}

pub(crate) fn to_byte(channel: f64) -> u8 {
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

/// Resolve the arguments into one set of settings before any rendering starts.
fn parse_args(args: &[String]) -> Result<Settings, String> {
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut samples = DEFAULT_SAMPLES;
    let mut output = PathBuf::from(DEFAULT_OUTPUT);
    let mut explicit_format = None;

    let mut index = 0;
    while index < args.len() {
        let option = args[index].as_str();
        let value = || {
            args.get(index + 1)
                .map(String::as_str)
                .ok_or_else(|| format!("{option} needs a value"))
        };
        match option {
            "--width" => width = parse_dimension(option, value()?)?,
            "--height" => height = parse_dimension(option, value()?)?,
            "--samples" => {
                let value = value()?;
                let count = value
                    .parse::<usize>()
                    .map_err(|_| format!("--samples: {value:?} is not a whole number"))?;
                if count < 1 {
                    return Err(format!("--samples: {value:?} is less than 1"));
                }
                samples = count;
            }
            "--output" => output = PathBuf::from(value()?),
            "--format" => {
                explicit_format = Some(match value()? {
                    "png" => Format::Png,
                    "ppm" => Format::Ppm,
                    other => return Err(format!("--format: unrecognised format {other:?}")),
                });
            }
            other => return Err(format!("unknown option {other:?}")),
        }
        index += 2;
    }

    let format = resolve_format(explicit_format, &output)?;
    Ok(Settings {
        width,
        height,
        samples,
        output,
        format,
    })
}

/// A pixel dimension, which must parse as a whole number of at least 2.
fn parse_dimension(option: &str, value: &str) -> Result<usize, String> {
    let dimension = value
        .parse::<usize>()
        .map_err(|_| format!("{option}: {value:?} is not a whole number"))?;
    if dimension < 2 {
        return Err(format!("{option}: {value:?} is less than 2"));
    }
    Ok(dimension)
}

/// An explicit format, or the one named by the output file's extension.
fn resolve_format(explicit: Option<Format>, output: &Path) -> Result<Format, String> {
    if let Some(format) = explicit {
        return Ok(format);
    }
    match output.extension().and_then(|extension| extension.to_str()) {
        Some("png") => Ok(Format::Png),
        Some("ppm") => Ok(Format::Ppm),
        _ => Err(format!(
            "{}: output name has no png or ppm extension and no --format is given",
            output.display()
        )),
    }
}

fn run(settings: &Settings) -> io::Result<()> {
    let pixels = render(settings.width, settings.height, settings.samples);
    let mut bytes = Vec::new();
    match settings.format {
        Format::Png => png::write_png(&mut bytes, settings.width, settings.height, &pixels)?,
        Format::Ppm => write_ppm(&mut bytes, settings.width, settings.height, &pixels)?,
    }
    fs::write(&settings.output, bytes)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let settings = match parse_args(&args) {
        Ok(settings) => settings,
        Err(message) => {
            eprintln!("error: {message}");
            eprintln!("{USAGE}");
            std::process::exit(2);
        }
    };
    if let Err(error) = run(&settings) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_top_row_is_sky() {
        let pixels = render(200, 100, 16);
        assert!(pixels[0].z > pixels[0].x, "the sky is bluer than it is red");
    }

    #[test]
    fn two_renders_with_the_same_options_give_the_same_pixels() {
        assert_eq!(render(20, 10, 2), render(20, 10, 2));
    }

    #[test]
    fn the_sample_count_changes_the_pixels() {
        assert_ne!(render(20, 10, 1), render(20, 10, 4));
    }
}
