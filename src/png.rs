//! Encode an image as a PNG file.

use std::io::{self, Write};

const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
const MAX_STORED_BLOCK: usize = 65535;

/// Write `rgb` as an 8-bit RGB PNG with no interlacing.
///
/// `rgb` holds `width * height * 3` bytes, three per pixel, row by row from the top.
pub fn write_png(out: &mut impl Write, width: usize, height: usize, rgb: &[u8]) -> io::Result<()> {
    out.write_all(&SIGNATURE)?;

    let mut ihdr = [0u8; 13];
    ihdr[0..4].copy_from_slice(&(width as u32).to_be_bytes());
    ihdr[4..8].copy_from_slice(&(height as u32).to_be_bytes());
    ihdr[8] = 8;
    ihdr[9] = 2;
    write_chunk(out, b"IHDR", &ihdr)?;
    write_chunk(out, b"IDAT", &zlib_stream(&scanlines(width, height, rgb)))?;
    write_chunk(out, b"IEND", &[])
}

/// Write one chunk: the data length, the kind, the data and the CRC-32 of kind and data.
fn write_chunk(out: &mut impl Write, kind: &[u8; 4], data: &[u8]) -> io::Result<()> {
    out.write_all(&(data.len() as u32).to_be_bytes())?;
    out.write_all(kind)?;
    out.write_all(data)?;
    out.write_all(&crc32(&[kind, data]).to_be_bytes())
}

/// Prefix each row with filter byte 0 (no filter) to form the image's scanline stream.
fn scanlines(width: usize, height: usize, rgb: &[u8]) -> Vec<u8> {
    let mut raw = Vec::with_capacity(height * (width * 3 + 1));
    for row in rgb.chunks_exact(width * 3) {
        raw.push(0);
        raw.extend_from_slice(row);
    }
    raw
}

/// Wrap `raw` in a zlib stream built from stored deflate blocks.
fn zlib_stream(raw: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    let blocks = raw.chunks(MAX_STORED_BLOCK).count();
    for (index, block) in raw.chunks(MAX_STORED_BLOCK).enumerate() {
        let last = index + 1 == blocks;
        out.push(u8::from(last));
        let length = block.len() as u16;
        out.extend_from_slice(&length.to_le_bytes());
        out.extend_from_slice(&(!length).to_le_bytes());
        out.extend_from_slice(block);
    }
    out.extend_from_slice(&adler32(raw).to_be_bytes());
    out
}

/// Compute the CRC-32 of the concatenated parts.
fn crc32(parts: &[&[u8]]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for part in parts {
        for &byte in *part {
            crc ^= byte as u32;
            for _ in 0..8 {
                let mask = (crc & 1).wrapping_neg();
                crc = (crc >> 1) ^ (0xedb8_8320 & mask);
            }
        }
    }
    !crc
}

/// Compute the Adler-32 checksum of the whole input.
fn adler32(data: &[u8]) -> u32 {
    let mut a = 1u32;
    let mut b = 0u32;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_two_pixel_image_is_a_valid_png() {
        let mut bytes = Vec::new();
        write_png(&mut bytes, 2, 1, &[255, 0, 0, 0, 127, 255]).unwrap();
        let expected: &[u8] = &[
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x7b, 0x40, 0xe8, 0xdd, 0x00, 0x00, 0x00, 0x12, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x01, 0x01, 0x07, 0x00, 0xf8, 0xff, 0x00, 0xff, 0x00, 0x00, 0x00, 0x7f, 0xff, 0x07,
            0xfe, 0x02, 0x7e, 0xa1, 0x8a, 0xbb, 0x7c, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e,
            0x44, 0xae, 0x42, 0x60, 0x82,
        ];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn a_tall_image_splits_into_stored_blocks() {
        let width = 200;
        let height = 110;
        let mut pixels = Vec::with_capacity(width * height * 3);
        for i in 0..width * height {
            match i % 3 {
                0 => pixels.extend_from_slice(&[255, 0, 0]),
                1 => pixels.extend_from_slice(&[0, 255, 0]),
                _ => pixels.extend_from_slice(&[0, 0, 255]),
            }
        }
        let mut bytes = Vec::new();
        write_png(&mut bytes, width, height, &pixels).unwrap();

        assert_eq!(&bytes[0..8], &SIGNATURE);

        let mut offset = 8;
        let mut chunks = Vec::new();
        while offset < bytes.len() {
            let length = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
            let kind = &bytes[offset + 4..offset + 8];
            let data = &bytes[offset + 8..offset + 8 + length];
            chunks.push((kind.to_vec(), data.to_vec()));
            offset += 12 + length;
        }

        let (ihdr_kind, ihdr) = &chunks[0];
        assert_eq!(ihdr_kind, b"IHDR");
        assert_eq!(ihdr.len(), 13);
        assert_eq!(
            u32::from_be_bytes(ihdr[0..4].try_into().unwrap()),
            width as u32
        );
        assert_eq!(
            u32::from_be_bytes(ihdr[4..8].try_into().unwrap()),
            height as u32
        );
        assert_eq!(ihdr[8], 8);
        assert_eq!(ihdr[9], 2);

        let idat: Vec<&(Vec<u8>, Vec<u8>)> = chunks
            .iter()
            .filter(|(kind, _)| kind.as_slice() == b"IDAT")
            .collect();
        assert_eq!(idat.len(), 1);
        let zlib = &idat[0].1;
        assert_eq!(&zlib[0..2], &[0x78, 0x01]);

        let mut position = 2;
        let mut payloads: Vec<Vec<u8>> = Vec::new();
        let mut lengths = Vec::new();
        loop {
            let header = zlib[position];
            let length = u16::from_le_bytes(zlib[position + 1..position + 3].try_into().unwrap());
            let complement =
                u16::from_le_bytes(zlib[position + 3..position + 5].try_into().unwrap());
            assert_eq!(complement, !length);
            lengths.push(length);
            position += 5;
            payloads.push(zlib[position..position + length as usize].to_vec());
            position += length as usize;
            if header & 1 == 1 {
                break;
            }
        }
        assert_eq!(lengths, vec![65535, 575]);

        let mut expected = Vec::new();
        for row in pixels.chunks_exact(width * 3) {
            expected.push(0);
            expected.extend_from_slice(row);
        }
        let joined: Vec<u8> = payloads.concat();
        assert_eq!(joined, expected);

        assert_eq!(chunks.last().unwrap().0, b"IEND");
        assert!(chunks.last().unwrap().1.is_empty());
    }
}
