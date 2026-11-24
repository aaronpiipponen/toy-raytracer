//! Command-line behaviour of the binary: output formats, PNG pixels and rejected input.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

/// A temporary directory removed when the test ends.
struct Scratch {
    dir: PathBuf,
}

impl Scratch {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("toy-raytracer-{}-{name}", std::process::id()));
        fs::create_dir_all(&dir).expect("create the scratch directory");
        Scratch { dir }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_toy-raytracer"))
        .args(args)
        .output()
        .expect("the binary runs")
}

#[test]
fn writes_a_png_whose_pixels_match_the_ppm_render() {
    let scratch = Scratch::new("png-match");
    let png_path = scratch.path("match.png");
    let result = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--samples",
        "1",
        "--output",
        png_path.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");
    assert!(
        result.stdout.is_empty(),
        "the image goes only to the output file"
    );
    assert!(
        !result.stderr.is_empty(),
        "the render reports progress on standard error"
    );

    let png_bytes = fs::read(&png_path).expect("the PNG exists");
    let (width, height, png_pixels) = decode_png(&png_bytes);
    assert_eq!((width, height), (8, 4));

    let ppm_path = scratch.path("match.ppm");
    let result = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--samples",
        "1",
        "--format",
        "ppm",
        "--output",
        ppm_path.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");

    let ppm_pixels = read_ppm(&fs::read_to_string(&ppm_path).expect("the PPM exists"));
    assert_eq!(ppm_pixels.len(), 32);
    for index in [0, 20, 31] {
        assert_eq!(
            png_pixels[index], ppm_pixels[index],
            "pixel {index} differs between the PNG and the PPM"
        );
    }
}

#[test]
fn rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit() {
    let scratch = Scratch::new("rejected");
    let output_path = scratch.path("rejected.png");
    let output = output_path.to_str().unwrap();
    let attempts = [
        vec!["--nonsense", "--output", output],
        vec!["--width", "wide", "--output", output],
        vec!["--samples", "0", "--output", output],
    ];
    for args in attempts {
        let result = run(&args);
        assert!(!result.status.success(), "{args:?} should fail");
        assert!(!result.stderr.is_empty(), "{args:?} should explain");
        assert!(!output_path.exists(), "{args:?} should write no file");
    }
}

#[test]
fn selects_the_format_from_the_output_extension_and_lets_an_explicit_format_win() {
    let scratch = Scratch::new("formats");

    let png_path = scratch.path("a.png");
    let result = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--output",
        png_path.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");
    let bytes = fs::read(&png_path).expect("the PNG exists");
    assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));

    let ppm_path = scratch.path("a.ppm");
    let result = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--output",
        ppm_path.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");
    let bytes = fs::read(&ppm_path).expect("the PPM exists");
    assert!(bytes.starts_with(b"P3"));

    let forced_path = scratch.path("b.png");
    let result = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--format",
        "ppm",
        "--output",
        forced_path.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");
    let bytes = fs::read(&forced_path).expect("the explicit PPM exists");
    assert!(bytes.starts_with(b"P3"));

    let unknown_path = scratch.path("a.jpg");
    let result = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--output",
        unknown_path.to_str().unwrap(),
    ]);
    assert!(!result.status.success(), "a .jpg output should fail");
    assert!(!result.stderr.is_empty());
    assert!(!unknown_path.exists());
}

/// Decode a PNG of exactly the writer's shape: 8-bit RGB, filter 0 and stored blocks.
fn decode_png(bytes: &[u8]) -> (u32, u32, Vec<[u8; 3]>) {
    assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]));

    let mut position = 8;
    let mut width = 0;
    let mut height = 0;
    let mut idat: Option<Vec<u8>> = None;
    let mut saw_end = false;
    while position < bytes.len() {
        let length = read_u32(bytes, position) as usize;
        let kind = &bytes[position + 4..position + 8];
        let data = &bytes[position + 8..position + 8 + length];
        let stored_crc = read_u32(bytes, position + 8 + length);
        let mut checksum_input = Vec::new();
        checksum_input.extend_from_slice(kind);
        checksum_input.extend_from_slice(data);
        assert_eq!(stored_crc, crc32(&checksum_input), "chunk CRC-32");
        position += 12 + length;

        match kind {
            b"IHDR" => {
                assert_eq!(length, 13);
                width = read_u32(data, 0);
                height = read_u32(data, 4);
                assert_eq!(&data[8..13], &[8, 2, 0, 0, 0]);
            }
            b"IDAT" => {
                assert!(idat.is_none(), "one IDAT chunk");
                idat = Some(data.to_vec());
            }
            b"IEND" => {
                saw_end = true;
                break;
            }
            _ => panic!("unexpected chunk"),
        }
    }
    assert!(saw_end, "the IEND chunk is present");

    let zlib = idat.expect("the IDAT chunk is present");
    assert_eq!(&zlib[..2], &[0x78, 0x01]);
    let mut raw = Vec::new();
    let mut position = 2;
    loop {
        let final_block = match zlib[position] {
            0x00 => false,
            0x01 => true,
            other => panic!("unexpected block flag {other}"),
        };
        let length = u16::from_le_bytes([zlib[position + 1], zlib[position + 2]]) as usize;
        let complement = u16::from_le_bytes([zlib[position + 3], zlib[position + 4]]);
        assert_eq!(
            complement,
            !(length as u16),
            "stored block length complement"
        );
        position += 5;
        raw.extend_from_slice(&zlib[position..position + length]);
        position += length;
        if final_block {
            break;
        }
    }
    assert_eq!(
        read_u32(&zlib, position),
        adler32(&raw),
        "the zlib Adler-32 trailer"
    );
    assert_eq!(position + 4, zlib.len());

    let row_length = 1 + width as usize * 3;
    assert_eq!(raw.len(), height as usize * row_length);
    let mut pixels = Vec::with_capacity(width as usize * height as usize);
    for row in raw.chunks_exact(row_length) {
        assert_eq!(row[0], 0, "scanline filter byte");
        for colour in row[1..].chunks_exact(3) {
            pixels.push([colour[0], colour[1], colour[2]]);
        }
    }
    (width, height, pixels)
}

/// Read the RGB triples of an ASCII PPM image.
fn read_ppm(text: &str) -> Vec<[u8; 3]> {
    let mut tokens = text.split_whitespace();
    assert_eq!(tokens.next(), Some("P3"));
    let width: usize = tokens.next().unwrap().parse().unwrap();
    let height: usize = tokens.next().unwrap().parse().unwrap();
    assert_eq!(tokens.next(), Some("255"));
    let mut pixels = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        let red: u8 = tokens.next().unwrap().parse().unwrap();
        let green: u8 = tokens.next().unwrap().parse().unwrap();
        let blue: u8 = tokens.next().unwrap().parse().unwrap();
        pixels.push([red, green, blue]);
    }
    assert!(
        tokens.next().is_none(),
        "no pixels beyond the declared size"
    );
    pixels
}

fn read_u32(bytes: &[u8], position: usize) -> u32 {
    u32::from_be_bytes([
        bytes[position],
        bytes[position + 1],
        bytes[position + 2],
        bytes[position + 3],
    ])
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    crc ^ 0xFFFF_FFFF
}

fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}
