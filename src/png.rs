//! A PNG writer that stores scanlines in uncompressed deflate blocks.

use std::io::{self, Write};

use crate::to_byte;
use crate::vec3::Colour;

const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];

/// The largest payload a stored deflate block can hold.
const MAX_STORED_BLOCK: usize = 65_535;

/// Write the pixels as an 8-bit RGB PNG with one IDAT chunk of stored deflate blocks.
pub fn write_png(
    out: &mut impl Write,
    width: usize,
    height: usize,
    pixels: &[Colour],
) -> io::Result<()> {
    out.write_all(&SIGNATURE)?;

    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&(width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.push(8); // Bit depth.
    header.push(2); // Colour type: truecolour RGB.
    header.push(0); // Compression method.
    header.push(0); // Filter method.
    header.push(0); // Interlace method.
    write_chunk(out, b"IHDR", &header)?;

    let mut raw = Vec::with_capacity(height * (1 + 3 * width));
    for row in 0..height {
        raw.push(0); // Filter type 0 for every scanline.
        for column in 0..width {
            let pixel = pixels[row * width + column];
            raw.push(to_byte(pixel.x));
            raw.push(to_byte(pixel.y));
            raw.push(to_byte(pixel.z));
        }
    }

    let mut zlib = Vec::new();
    zlib.extend_from_slice(&[0x78, 0x01]);
    let mut blocks = raw.chunks(MAX_STORED_BLOCK).peekable();
    while let Some(block) = blocks.next() {
        let final_block = blocks.peek().is_none();
        zlib.push(if final_block { 0x01 } else { 0x00 });
        let length = block.len() as u16;
        zlib.extend_from_slice(&length.to_le_bytes());
        zlib.extend_from_slice(&(!length).to_le_bytes());
        zlib.extend_from_slice(block);
    }
    zlib.extend_from_slice(&adler32(&raw).to_be_bytes());

    write_chunk(out, b"IDAT", &zlib)?;
    write_chunk(out, b"IEND", &[])?;
    Ok(())
}

/// Write one chunk: its length, type, data and the CRC-32 of the type and data.
fn write_chunk(out: &mut impl Write, kind: &[u8; 4], data: &[u8]) -> io::Result<()> {
    out.write_all(&(data.len() as u32).to_be_bytes())?;
    out.write_all(kind)?;
    out.write_all(data)?;
    out.write_all(&crc32(&[kind.as_slice(), data]).to_be_bytes())
}

/// The standard reflected CRC-32 used by every PNG chunk.
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

/// The Adler-32 checksum of the uncompressed data, as the zlib trailer stores it.
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
