//! Command-line behaviour of the binary: output formats, PNG pixels and rejected input.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];

/// Run the built binary with the given arguments.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_toy-raytracer"))
        .args(args)
        .output()
        .expect("the binary runs")
}

/// A scratch directory that removes itself when the test ends.
struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("toy-raytracer-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("the scratch directory is created");
        Scratch { path }
    }

    fn join(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn writes_a_png_whose_pixels_match_the_ppm_render() {
    let scratch = Scratch::new("pixels");
    let png_path = scratch.join("match.png");
    let ppm_path = scratch.join("match.ppm");
    let png_name = png_path.to_str().expect("the temp path is UTF-8");
    let ppm_name = ppm_path.to_str().expect("the temp path is UTF-8");

    let png_run = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--samples",
        "1",
        "--output",
        png_name,
    ]);
    assert!(
        png_run.status.success(),
        "the PNG run succeeds: {png_run:?}"
    );
    let ppm_run = run(&[
        "--width",
        "8",
        "--height",
        "4",
        "--samples",
        "1",
        "--format",
        "ppm",
        "--output",
        ppm_name,
    ]);
    assert!(
        ppm_run.status.success(),
        "the PPM run succeeds: {ppm_run:?}"
    );

    let (width, height, png_pixels) = decode_png(&fs::read(&png_path).expect("the PNG is written"));
    assert_eq!((width, height), (8, 4));
    let ppm_pixels = read_ppm(&fs::read(&ppm_path).expect("the PPM is written"));
    assert_eq!(ppm_pixels.len(), png_pixels.len());

    let center = (height as usize / 2) * width as usize + width as usize / 2;
    for index in [0, center, png_pixels.len() - 1] {
        assert_eq!(png_pixels[index], ppm_pixels[index], "pixel {index}");
    }
}

#[test]
fn rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit() {
    let scratch = Scratch::new("rejects");
    let path = scratch.join("rejected.png");
    let output = path.to_str().expect("the temp path is UTF-8");

    for args in [
        vec!["--nonsense", "--output", output],
        vec!["--width", "wide", "--output", output],
        vec!["--samples", "0", "--output", output],
    ] {
        let result = run(&args);
        assert!(
            !result.status.success(),
            "a non-zero exit for {args:?}: {result:?}"
        );
        assert!(
            !result.stderr.is_empty(),
            "an error on stderr for {args:?}: {result:?}"
        );
        assert!(!path.exists(), "no image file for {args:?}");
    }
}

#[test]
fn selects_the_format_from_the_output_extension_and_lets_an_explicit_format_win() {
    let scratch = Scratch::new("formats");
    let small = ["--width", "8", "--height", "4", "--samples", "1"];

    let png_path = scratch.join("a.png");
    let png_name = png_path.to_str().expect("the temp path is UTF-8");
    let png_run = run(&[&small[..], &["--output", png_name]].concat());
    assert!(
        png_run.status.success(),
        "the PNG run succeeds: {png_run:?}"
    );
    assert!(fs::read(&png_path)
        .expect("the PNG is written")
        .starts_with(&PNG_SIGNATURE));

    let ppm_path = scratch.join("a.ppm");
    let ppm_name = ppm_path.to_str().expect("the temp path is UTF-8");
    let ppm_run = run(&[&small[..], &["--output", ppm_name]].concat());
    assert!(
        ppm_run.status.success(),
        "the PPM run succeeds: {ppm_run:?}"
    );
    assert!(fs::read(&ppm_path)
        .expect("the PPM is written")
        .starts_with(b"P3"));

    let explicit_path = scratch.join("b.png");
    let explicit_name = explicit_path.to_str().expect("the temp path is UTF-8");
    let explicit_run = run(&[&small[..], &["--format", "ppm", "--output", explicit_name]].concat());
    assert!(
        explicit_run.status.success(),
        "the explicit format run succeeds: {explicit_run:?}"
    );
    assert!(fs::read(&explicit_path)
        .expect("the explicit output is written")
        .starts_with(b"P3"));

    let unknown_path = scratch.join("a.jpg");
    let unknown_name = unknown_path.to_str().expect("the temp path is UTF-8");
    let unknown_run = run(&[&small[..], &["--output", unknown_name]].concat());
    assert!(
        !unknown_run.status.success(),
        "an unknown extension fails: {unknown_run:?}"
    );
    assert!(!unknown_path.exists(), "no file for an unknown extension");
}

/// Decode exactly the PNG this writer produces and return its pixels in row order.
fn decode_png(bytes: &[u8]) -> (u32, u32, Vec<[u8; 3]>) {
    assert_eq!(&bytes[..8], &PNG_SIGNATURE, "the PNG signature");

    let mut offset = 8;
    let mut dimensions = None;
    let mut idat = Vec::new();
    let mut idat_chunks = 0;
    let mut seen_iend = false;
    while offset < bytes.len() {
        let length = u32::from_be_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .expect("a chunk length"),
        ) as usize;
        let kind = &bytes[offset + 4..offset + 8];
        let data = &bytes[offset + 8..offset + 8 + length];
        let crc = u32::from_be_bytes(
            bytes[offset + 8 + length..offset + 12 + length]
                .try_into()
                .expect("a chunk CRC"),
        );
        assert_eq!(crc, crc32(&[kind, data]), "the chunk CRC");
        offset += 12 + length;

        match kind {
            b"IHDR" => {
                assert_eq!(length, 13, "the IHDR length");
                let width = u32::from_be_bytes(data[0..4].try_into().expect("a width"));
                let height = u32::from_be_bytes(data[4..8].try_into().expect("a height"));
                assert_eq!(data[8], 8, "the bit depth");
                assert_eq!(data[9], 2, "the colour type");
                assert_eq!(data[10], 0, "the compression method");
                assert_eq!(data[11], 0, "the filter method");
                assert_eq!(data[12], 0, "the interlace method");
                dimensions = Some((width, height));
            }
            b"IDAT" => {
                idat_chunks += 1;
                idat.extend_from_slice(data);
            }
            b"IEND" => {
                assert_eq!(length, 0, "the IEND length");
                seen_iend = true;
                break;
            }
            other => panic!("unexpected chunk {:?}", String::from_utf8_lossy(other)),
        }
    }
    assert_eq!(idat_chunks, 1, "exactly one IDAT chunk");
    assert!(seen_iend, "the IEND chunk is written");

    let (width, height) = dimensions.expect("the IHDR chunk is written");
    let raw = inflate_stored(&idat);
    assert_eq!(
        raw.len(),
        height as usize * (1 + 3 * width as usize),
        "the scanline byte count"
    );

    let mut pixels = Vec::new();
    for row in raw.chunks_exact(1 + 3 * width as usize) {
        assert_eq!(row[0], 0, "the filter type");
        for pixel in row[1..].chunks_exact(3) {
            pixels.push([pixel[0], pixel[1], pixel[2]]);
        }
    }
    (width, height, pixels)
}

/// Read the stored deflate blocks of exactly this writer's zlib stream.
fn inflate_stored(zlib: &[u8]) -> Vec<u8> {
    assert_eq!(&zlib[..2], &[0x78, 0x01], "the zlib header");
    let mut offset = 2;
    let mut raw = Vec::new();
    loop {
        let header = zlib[offset];
        assert_eq!(header & 0b1111_1110, 0, "a stored block header");
        let final_block = header & 1 == 1;
        let length = u16::from_le_bytes([zlib[offset + 1], zlib[offset + 2]]);
        let complement = u16::from_le_bytes([zlib[offset + 3], zlib[offset + 4]]);
        assert_eq!(!length, complement, "the block length complement");
        let length = length as usize;
        offset += 5;
        raw.extend_from_slice(&zlib[offset..offset + length]);
        offset += length;
        if final_block {
            break;
        }
    }
    let trailer = u32::from_be_bytes(zlib[offset..offset + 4].try_into().expect("the Adler-32"));
    assert_eq!(trailer, adler32(&raw), "the Adler-32 trailer");
    assert_eq!(offset + 4, zlib.len(), "no bytes after the zlib trailer");
    raw
}

fn crc32(parts: &[&[u8]]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for part in parts {
        for &byte in *part {
            crc ^= u32::from(byte);
            for _ in 0..8 {
                let mask = (crc & 1).wrapping_neg();
                crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
            }
        }
    }
    !crc
}

fn adler32(bytes: &[u8]) -> u32 {
    const MODULUS: u32 = 65_521;
    let mut a = 1u32;
    let mut b = 0u32;
    for &byte in bytes {
        a = (a + u32::from(byte)) % MODULUS;
        b = (b + a) % MODULUS;
    }
    (b << 16) | a
}

/// Read the P3 text this writer writes and return its pixels in row order.
fn read_ppm(bytes: &[u8]) -> Vec<[u8; 3]> {
    let text = std::str::from_utf8(bytes).expect("the PPM is text");
    let mut tokens = text.split_whitespace();
    assert_eq!(tokens.next(), Some("P3"), "the PPM magic");
    let width: usize = tokens
        .next()
        .expect("a width")
        .parse()
        .expect("a numeric width");
    let height: usize = tokens
        .next()
        .expect("a height")
        .parse()
        .expect("a numeric height");
    assert_eq!(tokens.next(), Some("255"), "the PPM maximum");

    let channels: Vec<u8> = tokens
        .map(|token| token.parse().expect("a channel byte"))
        .collect();
    let pixels: Vec<[u8; 3]> = channels
        .chunks_exact(3)
        .map(|pixel| [pixel[0], pixel[1], pixel[2]])
        .collect();
    assert_eq!(pixels.len(), width * height, "the pixel count");
    pixels
}
