# Image output

The program writes an 8-bit RGB image in PNG or PPM format, with each channel converted from a colour value in 0 to 1.

## Channel conversion

The renderer gives each pixel a red, green and blue value in the range 0 to 1. Every channel becomes 1 byte through four steps:

1. Clamp the value to the range 0 to 1.
2. Take the square root, the gamma 2 correction.
3. Multiply by 255.999.
4. Truncate the fractional part.

```text
byte = floor(255.999 * sqrt(clamp(channel, 0, 1)))
```

A value below 0 or above 1 is clamped before the square root, so every byte lies in 0 to 255. For how a pixel's colour values are computed, see [Rendering](/docs/rendering.md).

Both formats use this conversion. For one rendered pixel, the PNG channel bytes and the PPM channel values are equal.

## PPM layout

The PPM file holds plain text in the `P3` form. It starts with the magic `P3` on its own line, then a line with the width and the height, then a line with the maximum value `255`, and each of the three lines ends in a newline. After the header comes a line per pixel, holding the red, green and blue bytes as decimal numbers separated by a space, in row-major order from the top row. Each byte is written without padding.

## PNG layout

The PNG writer produces four parts in order: the signature, one IHDR chunk, one IDAT chunk and one IEND chunk. It builds the whole file in memory before the program writes it to the output path.

### Chunks

Every chunk holds four fields in order:

| Field | Size | Content |
| --- | --- | --- |
| Data length | 4 bytes | number of data bytes, big-endian |
| Type | 4 bytes | chunk name in ASCII |
| Data | the data length | chunk payload |
| CRC-32 | 4 bytes | CRC-32 of the type and the data, big-endian |

The CRC-32 starts at `0xffffffff`, uses the reflected polynomial `0xedb88320`, and is complemented at the end. The writer emits the chunk types `IHDR`, `IDAT` and `IEND`.

### Signature and header

The file starts with the 8 signature bytes `89 50 4e 47 0d 0a 1a 0a`.

The IHDR chunk holds 13 data bytes:

| Byte offset | Field | Value |
| --- | --- | --- |
| 0 | width | image width, 4-byte big-endian |
| 4 | height | image height, 4-byte big-endian |
| 8 | bit depth | 8 |
| 9 | colour type | 2, truecolour RGB |
| 10 | compression method | 0 |
| 11 | filter method | 0 |
| 12 | interlace method | 0 |

### Scanlines

The IDAT data carries the image scanlines. Each row starts with filter type 0, followed by 3 bytes per pixel in left-to-right order. A row is `1 + 3 * width` bytes and the whole image is `height * (1 + 3 * width)` bytes. Every row uses filter type 0, so a decoder reconstructs each pixel without filtering.

### Zlib stream

The IDAT data is one zlib stream in three parts:

1. 2 header bytes, `0x78` and `0x01`.
2. One or more stored deflate blocks that carry the scanline bytes in order.
3. The Adler-32 of the scanline bytes, 4 bytes big-endian.

Each stored block starts with a header byte. Its lowest bit marks the final block and the other bits are zero, so the byte is `0x00` for every block except the last, which is `0x01`. The block then holds its length as a 2-byte little-endian number, the ones' complement of that length as another 2-byte little-endian number, and the data bytes. A block carries at most 65,535 data bytes.

The Adler-32 covers every scanline byte, including the filter bytes. It sums with modulus 65,521 from initial values 1 and 0.

### End of the file

The IEND chunk closes the file and carries no data, so its length field is 0 and its CRC-32 covers the type alone.

## File size

Stored blocks leave the pixel data uncompressed, so the file size follows from the dimensions. The width and height come from the options in [Command line](/docs/command-line.md). For a width `w` and a height `h`:

```text
raw    = h * (1 + 3 * w)
blocks = (raw + 65534) / 65535
size   = 63 + 5 * blocks + raw
```

The 63 bytes are the 8-byte signature, the 25-byte IHDR chunk, the 12 bytes of IDAT framing, the 12-byte IEND chunk, the 2-byte zlib header and the 4-byte Adler-32 trailer. Each stored block adds 5 bytes.

A 400 by 200 image has 240,200 scanline bytes in four blocks, so the file is 240,283 bytes, close to 3 bytes per pixel.

## Pixel order

Both formats store the same pixels in the same order: row-major from the top row, with red, then green, then blue for each pixel. The first pixel in either file is the top-left pixel of the image.
