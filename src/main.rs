//! Render a fixed scene and write it to a file as a PNG or PPM image.

mod camera;
mod hittable;
mod png;
mod ray;
// Seeded randomness for the scattering and sampling the plans add.
#[allow(dead_code)]
mod rng;
mod sphere;
mod vec3;

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::{Colour, Vec3};

const DEFAULT_WIDTH: usize = 400;
const DEFAULT_HEIGHT: usize = 200;
const DEFAULT_SAMPLES: usize = 16;
const DEFAULT_OUTPUT: &str = "render.png";

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
    world
}

/// Every pixel's colour, row by row from the top, averaging `samples` rays per pixel.
fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let u = column as f64 / (width - 1) as f64;
            let v = row as f64 / (height - 1) as f64;
            let ray = camera.ray(u, v);
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                colour = colour + ray_colour(&ray, &world);
            }
            pixels.push(colour / samples as f64);
        }
    }
    pixels
}

pub(crate) fn to_byte(channel: f64) -> u8 {
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
    fn the_center_pixel_shows_the_sphere_facing_the_camera() {
        let pixels = render(200, 100, 16);
        let center = pixels[(100 / 2) * 200 + 200 / 2];
        assert!(center.z > 0.9, "the normal faces the camera: {center:?}");
    }

    #[test]
    fn the_top_row_is_sky() {
        let pixels = render(200, 100, 16);
        assert!(pixels[0].z > pixels[0].x, "the sky is bluer than it is red");
    }
}
