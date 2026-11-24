//! A PNG writer that stores scanlines in uncompressed deflate blocks.

use std::io::{self, Write};

use crate::to_byte;
use crate::vec3::Colour;

/// Write an image as an 8-bit truecolour PNG without interlacing or compression.
pub fn write_png(
    out: &mut impl Write,
    width: usize,
    height: usize,
    pixels: &[Colour],
) -> io::Result<()> {
    out.write_all(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a])?;

    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&(width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.extend_from_slice(&[8, 2, 0, 0, 0]);
    write_chunk(out, b"IHDR", &header)?;

    let mut raw = Vec::with_capacity(height * (1 + width * 3));
    for row in 0..height {
        raw.push(0);
        for column in 0..width {
            let pixel = pixels[row * width + column];
            raw.push(to_byte(pixel.x));
            raw.push(to_byte(pixel.y));
            raw.push(to_byte(pixel.z));
        }
    }

    let mut zlib = Vec::new();
    zlib.extend_from_slice(&[0x78, 0x01]);
    let mut offset = 0;
    while offset < raw.len() {
        let length = (raw.len() - offset).min(65535);
        let final_block = offset + length == raw.len();
        zlib.push(if final_block { 0x01 } else { 0x00 });
        let length_field = length as u16;
        zlib.extend_from_slice(&length_field.to_le_bytes());
        zlib.extend_from_slice(&(!length_field).to_le_bytes());
        zlib.extend_from_slice(&raw[offset..offset + length]);
        offset += length;
    }
    zlib.extend_from_slice(&adler32(&raw).to_be_bytes());
    write_chunk(out, b"IDAT", &zlib)?;

    write_chunk(out, b"IEND", &[])
}

/// Write one chunk: its data length, its type, its data and its CRC-32.
fn write_chunk(out: &mut impl Write, kind: &[u8; 4], data: &[u8]) -> io::Result<()> {
    out.write_all(&(data.len() as u32).to_be_bytes())?;
    out.write_all(kind)?;
    out.write_all(data)?;
    let mut checksum_input = Vec::with_capacity(4 + data.len());
    checksum_input.extend_from_slice(kind);
    checksum_input.extend_from_slice(data);
    out.write_all(&crc32(&checksum_input).to_be_bytes())
}

/// The standard reflected CRC-32 used by PNG chunks.
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

/// The Adler-32 checksum of the uncompressed data, for the zlib trailer.
fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}
