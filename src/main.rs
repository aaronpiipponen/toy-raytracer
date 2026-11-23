//! Render a fixed scene and write it as a PNG or PPM image.

mod camera;
mod hittable;
mod png;
mod ray;
#[allow(dead_code)]
mod rng;
mod sphere;
mod vec3;

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::{Colour, Vec3};

const WIDTH: usize = 400;
const HEIGHT: usize = 200;
const DEFAULT_SAMPLES: usize = 16;
const DEFAULT_OUTPUT: &str = "render.png";

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

/// Return every pixel's colour, row by row from the top. A progress line on standard error reports
/// the share of rows done.
fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut pixels = Vec::with_capacity(width * height);
    for (done, row) in (0..height).rev().enumerate() {
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
        eprint!("\r{:3}%", (done + 1) * 100 / height);
    }
    eprintln!();
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    Png,
    Ppm,
}

#[derive(Debug, PartialEq)]
struct Config {
    width: usize,
    height: usize,
    samples: usize,
    output: String,
    format: Format,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: WIDTH,
            height: HEIGHT,
            samples: DEFAULT_SAMPLES,
            output: DEFAULT_OUTPUT.to_string(),
            format: Format::Png,
        }
    }
}

impl Config {
    /// Parse the command-line options, rejecting an unknown option, a missing value and a value
    /// the option cannot take, including an output path whose extension names no format when
    /// `--format` is absent.
    fn parse(args: impl Iterator<Item = String>) -> Result<Config, String> {
        let mut config = Config::default();
        let mut explicit_format: Option<Format> = None;
        let mut args = args;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--width" => config.width = parse_number(args.next(), "--width")?,
                "--height" => config.height = parse_number(args.next(), "--height")?,
                "--samples" => config.samples = parse_number(args.next(), "--samples")?,
                "--output" => config.output = parse_value(args.next(), "--output")?,
                "--format" => {
                    explicit_format = Some(match args.next().as_deref() {
                        Some("png") => Format::Png,
                        Some("ppm") => Format::Ppm,
                        _ => return Err("--format takes png or ppm".to_string()),
                    });
                }
                other => return Err(format!("unknown option: {other}")),
            }
        }
        if config.width < 2 {
            return Err("--width must be at least 2".to_string());
        }
        if config.height < 2 {
            return Err("--height must be at least 2".to_string());
        }
        if config.samples < 1 {
            return Err("--samples must be at least 1".to_string());
        }
        config.format = match explicit_format {
            Some(format) => format,
            None => match Path::new(&config.output)
                .extension()
                .and_then(|e| e.to_str())
            {
                Some("png") => Format::Png,
                Some("ppm") => Format::Ppm,
                _ => return Err("--output must end in .png or .ppm".to_string()),
            },
        };
        Ok(config)
    }
}

fn parse_value(value: Option<String>, option: &str) -> Result<String, String> {
    value.ok_or_else(|| format!("{option} needs a value"))
}

fn parse_number(value: Option<String>, option: &str) -> Result<usize, String> {
    parse_value(value, option)?
        .parse()
        .map_err(|_| format!("{option} needs a number"))
}

fn usage() -> &'static str {
    "usage: toy-raytracer [--width N] [--height N] [--samples N] [--output PATH] [--format png|ppm]"
}

fn main() -> io::Result<()> {
    let config = match Config::parse(std::env::args().skip(1)) {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };
    let pixels = render(config.width, config.height, config.samples);
    let mut out = io::BufWriter::new(File::create(&config.output)?);
    match config.format {
        Format::Png => {
            let rgb: Vec<u8> = pixels
                .iter()
                .flat_map(|p| [to_byte(p.x), to_byte(p.y), to_byte(p.z)])
                .collect();
            png::write_png(&mut out, config.width, config.height, &rgb)?;
        }
        Format::Ppm => write_ppm(&mut out, config.width, config.height, &pixels)?,
    }
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_center_pixel_shows_the_sphere_facing_the_camera() {
        let pixels = render(WIDTH, HEIGHT, 1);
        let center = pixels[(HEIGHT / 2) * WIDTH + WIDTH / 2];
        assert!(center.z > 0.9, "the normal faces the camera: {center:?}");
    }

    #[test]
    fn the_top_row_is_sky() {
        let pixels = render(WIDTH, HEIGHT, 1);
        assert!(pixels[0].z > pixels[0].x, "the sky is bluer than it is red");
    }

    #[test]
    fn the_defaults_are_400_by_200_with_16_samples() {
        let config = Config::parse(std::iter::empty::<String>()).unwrap();
        assert_eq!(config.width, 400);
        assert_eq!(config.height, 200);
        assert_eq!(config.samples, 16);
        assert_eq!(config.output, "render.png");
        assert_eq!(config.format, Format::Png);
    }

    #[test]
    fn a_full_command_line_sets_every_option() {
        let args = [
            "--width",
            "80",
            "--height",
            "40",
            "--samples",
            "4",
            "--output",
            "scene.ppm",
            "--format",
            "png",
        ];
        let config = Config::parse(args.into_iter().map(String::from)).unwrap();
        assert_eq!(config.width, 80);
        assert_eq!(config.height, 40);
        assert_eq!(config.samples, 4);
        assert_eq!(config.output, "scene.ppm");
        assert_eq!(config.format, Format::Png);
    }

    #[test]
    fn an_unknown_option_is_rejected() {
        let config = Config::parse(["--bogus".to_string()].into_iter());
        assert!(config.is_err());
    }
}
