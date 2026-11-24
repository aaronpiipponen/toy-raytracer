# Rendering one image

An image is produced by tracing one camera ray per sample through each pixel, averaging the sample colours, and encoding the result as 8-bit channels.

The renderer builds the fixed scene, creates the camera once and creates one random generator for the whole image. It then visits every pixel in row order and writes the encoded pixels to the output file.

## The camera

The camera sits at the origin, the point (0, 0, 0), and looks down the negative z axis. Its viewport is the plane at z = -1 and has a height of 2. The viewport width is twice the image aspect ratio, where the aspect ratio is the image width divided by the image height. A 400 by 200 image has a viewport 4 units wide.

A point on the viewport is given by two coordinates, `u` and `v`, each from 0 to 1. The point at `u` 0 and `v` 0 is the lower left corner of the viewport. The point at `u` 1 and `v` 1 is the upper right corner. A camera ray starts at the origin and passes through the viewport point.

## Sampling a pixel

Each pixel averages the colours of `--samples` camera rays. The default count is 16 and the smallest allowed count is 1. The option is described on the [command line page](/docs/command-line.md).

For each sample the renderer draws two random numbers in the range 0 to 1. It adds the first to the pixel's column and the second to the pixel's row. Dividing the column sum by the image width and the row sum by the image height gives `u` and `v`. The offsets place the sample at a random point inside the pixel.

The pixel colour is the sum of its sample colours divided by the sample count. The count changes the image. The fixed scene rendered with one sample and with four samples produces different pixels.

## The fixed scene

The scene holds five objects. Every render builds the same five.

| Object | Geometry | Material | Position and size |
|---|---|---|---|
| Diffuse sphere | Sphere | Diffuse, albedo (0.7, 0.3, 0.3) | Centre (0, 0, -1), radius 0.5 |
| Ground sphere | Sphere | Diffuse, albedo (0.8, 0.8, 0.8) | Centre (0, -100.5, -1), radius 100 |
| Metal sphere | Sphere | Metal, albedo (0.8, 0.8, 0.8), fuzz 0.3 | Centre (0.85, -0.25, -1.3), radius 0.25 |
| Glass sphere | Sphere | Glass, refractive index 1.5 | Centre (-1.1, 0, -1.5), radius 0.5 |
| Triangle | Triangle | Diffuse, albedo (0.7, 0.3, 0.3) | Vertices (0.6, 0.15, -1), (1.95, 0.15, -1), (1.275, 0.92, -1) |

Spheres and triangles are described on the [geometry page](/docs/geometry.md).

## The path of a ray

A sample's trace starts with the colour (1, 1, 1) and takes at most 50 bounces. Each bounce asks the scene for the nearest hit at a ray parameter greater than 0.001.

### A hit and a scatter

When the scene finds a hit, the material returns an attenuation colour and a scattered ray, or it reports that the ray is absorbed. With an attenuation, the running colour is multiplied by the attenuation channel by channel, and the scattered ray becomes the ray for the next step. With absorption, the sample is black at once. The [materials page](/docs/materials.md) describes each material.

### The sky colour

When the scene finds no hit, the sample colour is the running colour multiplied by the sky colour. The sky colour blends white with the blue (0.5, 0.7, 1). The blend factor is half of the unit direction's y component plus one half. A ray pointing straight down sees white, a ray pointing straight up sees (0.5, 0.7, 1), and a horizontal ray sees (0.75, 0.85, 1).

### The bounce limit

A ray that scatters on all 50 bounces without a miss or an absorption ends as black. A sample never includes more than 50 material scatters.

### The sample path

The diagram shows the path of one sample from its jittered position in the pixel to its contribution to the image file.

![A jittered position becomes a camera ray, the scene list finds the nearest hit, a hit scatters at the material and repeats for at most 50 bounces, a miss gives the sky colour and an absorbed ray is black, then the sample joins the average, is gamma encoded and written to the image file.](/docs/diagrams/render-pipeline.svg)

## Gamma encoding

Each channel of a pixel colour becomes one byte. The value is clamped to the range 0 to 1. The square root of the value applies the gamma 2 correction. The square root is multiplied by 255.999 and the product is truncated to a whole number. Both output writers use this conversion, described on the [image output page](/docs/image-output.md).

## Row order and the progress line

Rows are rendered from the top of the image to the bottom. The top row is processed first, and its pixels are the first written to the file.

After each row the renderer writes the share of finished rows to standard error. The share is the whole-number percentage of finished rows, the number of finished rows times 100 divided by the image height. A carriage return starts each update, so the update overwrites the line in place. The percentage is a whole number and drops the fractional part, and the last update reads 100%. After the last row a newline ends the line. Standard output is empty.

## Reproducibility

One generator serves the whole render. It is a 64-bit `xorshift64*` generator seeded with the fixed value 7. The same seed gives the same sequence.

The renderer draws from the generator for the pixel offsets and for the material scatters, in the order of rows, columns, samples and bounces. The seed is a constant in the source, and no command-line option changes it.

Two renders with the same width, height and sample count produce identical pixels. The same output format produces identical file bytes. The [requirements](/docs/requirements.md) state that the same options give the same bytes.
