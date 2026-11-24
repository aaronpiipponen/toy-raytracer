# Image output

Each rendered pixel colour becomes three bytes in a PNG or PPM file, and the output format comes from an option or the output path.

The pixel colours this page encodes are produced by the render described in [rendering](/docs/rendering.md). Both formats store the same three bytes for each pixel of a given render.

## Channel encoding

A rendered pixel holds three linear colour channels as floating-point numbers. Both writers convert every channel to one byte with the same four steps.

| Step | Operation |
|---|---|
| Clamp | Values below 0 become 0 and values above 1 become 1 |
| Gamma | The square root of the clamped value |
| Scale | The result multiplied by `255.999` |
| Truncate | The integer part of the product, as an unsigned byte |

The square root applies gamma 2 correction to the output. For a value between 0 and 1 the square root is larger than the value, so a mid-tone is stored with a higher byte than direct scaling would give. A channel of 0 becomes the byte 0, and a channel of 1 becomes 255 after the product `255.999` is truncated.

The PNG and PPM writers share this conversion, so the two formats record identical colour values for the same render. The command-line tests decode both formats and compare selected pixels.

## Choosing the format

The output format is resolved in this order.

1. An explicit `--format png` or `--format ppm` selects that format, whatever the output path says.
2. With no explicit format, the output file's extension selects the format. The extension `png` selects PNG, and the extension `ppm` selects PPM.
3. Any other extension, or an output path with no extension, is an error. The run prints the error and a usage line and exits with a non-zero status before it renders anything, so it writes no file.

The extension match is exact, so `.PNG` in capitals does not select PNG.

The option defaults and the checks on option values are in [running the program](/docs/running.md).

## PNG output

The PNG writer produces an 8-bit truecolour image. The file consists of the eight-byte signature, an `IHDR` chunk, one `IDAT` chunk and one `IEND` chunk, in that order.

### Chunk structure

Every chunk has the same four fields.

| Field | Size | Content |
|---|---|---|
| Length | 4 bytes, big-endian | The number of data bytes that follow |
| Type | 4 bytes | The chunk name |
| Data | The declared length | The chunk payload |
| CRC-32 | 4 bytes, big-endian | The standard reflected CRC-32 of the type and the data |

The CRC-32 uses the polynomial `0xEDB88320`. The signature before the chunks is `89 50 4E 47 0D 0A 1A 0A`.

| Chunk | Data |
|---|---|
| `IHDR` | Image width, image height and the five format fields below, 13 bytes in all |
| `IDAT` | The zlib stream described below |
| `IEND` | Empty, and ends the file |

The `IHDR` chunk records these values.

| Field | Value |
|---|---|
| Bit depth | 8 |
| Colour type | 2, truecolour red, green and blue |
| Compression method | 0 |
| Filter method | 0 |
| Interlace method | 0 |

### Scanline data

The image is stored row by row, with the top row first. Each row starts with one filter byte, which is 0 and selects no filtering, followed by the red, green and blue bytes of each pixel from left to right. The scanline data is therefore `height * (1 + 3 * width)` bytes long.

### The zlib stream

The file has exactly one `IDAT` chunk, and it holds a single zlib stream.

| Part | Content |
|---|---|
| Header | Two bytes, `0x78` and `0x01` |
| Body | Stored deflate blocks holding the scanline data |
| Trailer | The Adler-32 checksum of the scanline data, 4 bytes big-endian |

A stored deflate block carries the bytes as they are. Each block starts with a five-byte header: one flag byte, the two-byte little-endian length of the block, then the one's complement of that length in two bytes. The flag byte is `0x01` on the last block and `0x00` on every other block. The block body follows the header.

A block holds at most 65535 bytes, so the scanline data is split across `ceil(raw / 65535)` blocks, where `raw` is the byte length stated above. Each block adds five header bytes to the data it carries, so the `IDAT` data is larger than the scanline data by those headers and the zlib header and trailer. A PNG written this way is larger than one whose deflate blocks compress the scanline data.

## PPM output

The PPM writer produces an ASCII image in the `P3` form.

```text
P3
2 2
255
0 0 0
64 128 255
20 30 40
255 255 255
```

The header gives the form, then the width and height, then the maximum channel value, which is 255. One line for each pixel follows, holding the three channel bytes as decimal numbers separated by spaces, from the top row to the bottom row and left to right.

## Writing the output file

A run builds the whole image in memory before it touches the output path. Rendering produces every pixel, the writer encodes those pixels into a byte buffer, and one write then creates or replaces the file. The image goes only to that file, and nothing is written to standard output.

A run that stops before rendering completes makes no write.
