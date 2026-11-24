//! Render a fixed scene and write it to a file as a PNG or PPM image.

mod aabb;
mod camera;
mod hittable;
mod material;
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
use plane::Plane;
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

/// Return the colour a ray sees: its scattered path attenuated by each material, the sky gradient
/// when it misses.
fn ray_colour(ray: &Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour {
    let mut ray = *ray;
    let mut colour = Vec3::new(1.0, 1.0, 1.0);
    for _ in 0..MAX_BOUNCES {
        // The 0.001 lower bound is in ray-parameter units. It keeps a scattered ray from
        // hitting the surface it left.
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
    world.add(Box::new(Plane::new(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))),
    )));
    world.add(Box::new(Triangle::new(
        Vec3::new(-2.0, -0.5, -2.0),
        Vec3::new(2.0, -0.5, -2.0),
        Vec3::new(0.0, 1.5, -2.0),
        Rc::new(Lambertian::new(Colour::new(0.35, 0.5, 0.25))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.85, -0.25, -1.3),
        0.25,
        Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-0.85, -0.1, -1.3),
        0.4,
        Rc::new(Dielectric::new(1.5)),
    )));
    world
}

/// Return every pixel's colour, row by row from the top, averaging `samples` jittered rays per
/// pixel. One seeded generator supplies the jitter and every bounce, so a fixed size and sample
/// count give the same image. Reports the share of rows done on standard error.
fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut rng = Rng::new(SEED);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (column as f64 + rng.next_f64() - 0.5) / (width - 1) as f64;
                let v = (row as f64 + rng.next_f64() - 0.5) / (height - 1) as f64;
                colour = colour + ray_colour(&camera.ray(u, v), &world, &mut rng);
            }
            pixels.push(colour / samples as f64);
        }
        eprint!("\r{} of {} rows", height - row, height);
    }
    eprintln!();
    pixels
}

/// Return the byte for a linear channel in 0..=1 after clamping and gamma 2 correction.
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

/// Resolve the command line into the settings one render uses.
fn parse_args(args: &[String]) -> Result<Settings, String> {
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut samples = DEFAULT_SAMPLES;
    let mut output = PathBuf::from(DEFAULT_OUTPUT);
    let mut explicit_format: Option<Format> = None;

    let mut index = 0;
    while index < args.len() {
        let option = args[index].as_str();
        let value = args.get(index + 1);
        match (option, value) {
            ("--width", Some(value)) => width = parse_number(option, value, 2)?,
            ("--height", Some(value)) => height = parse_number(option, value, 2)?,
            ("--samples", Some(value)) => samples = parse_number(option, value, 1)?,
            ("--output", Some(value)) => output = PathBuf::from(value),
            ("--format", Some(value)) => {
                explicit_format = Some(match value.as_str() {
                    "png" => Format::Png,
                    "ppm" => Format::Ppm,
                    _ => return Err(format!("unknown format {value}")),
                });
            }
            ("--width" | "--height" | "--samples" | "--output" | "--format", None) => {
                return Err(format!("{option} needs a value"));
            }
            _ => return Err(format!("unknown option {option}")),
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

/// Parse an option's value as a count of at least `minimum`.
fn parse_number(option: &str, value: &str, minimum: usize) -> Result<usize, String> {
    let number = value
        .parse::<usize>()
        .map_err(|_| format!("{option} needs a number, not {value}"))?;
    if number < minimum {
        return Err(format!("{option} needs at least {minimum}, not {value}"));
    }
    Ok(number)
}

/// Choose the format: an explicit `--format` wins, otherwise the output extension decides.
fn resolve_format(explicit: Option<Format>, output: &Path) -> Result<Format, String> {
    if let Some(format) = explicit {
        return Ok(format);
    }
    match output.extension().and_then(|extension| extension.to_str()) {
        Some("png") => Ok(Format::Png),
        Some("ppm") => Ok(Format::Ppm),
        _ => Err(format!("cannot choose a format for {}", output.display())),
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
        let pixels = render(40, 20, 4);
        for column in [0, 20, 39] {
            let pixel = pixels[column];
            assert!(
                pixel.z > pixel.x,
                "the sky is bluer than it is red at column {column}"
            );
        }
    }

    #[test]
    fn the_same_settings_render_the_same_pixels() {
        assert_eq!(render(40, 20, 4), render(40, 20, 4));
    }

    #[test]
    fn the_sample_count_changes_the_pixel_colours() {
        assert_ne!(render(40, 20, 1), render(40, 20, 4));
    }
}
