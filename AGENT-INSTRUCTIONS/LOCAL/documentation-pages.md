The toy raytracer's documentation is `README.md`, the pages under `docs/`, the diagram under
`docs/diagrams/`, the rendered scene image at `docs/images/fixed-scene.png` and the logo at
`docs/brand/logo.svg`. The workspace publishes nothing, so it has no export, public checkout,
deployment, documentation index, screenshot capture setup or translated interface strings. Every
page is read in the repository.

- `README.md` is the product overview and the first page.
- `docs/requirements.md` states what the program does.
- `docs/rendering.md` covers the camera, samples, the path loop, the sky, gamma encoding and
  reproducibility, and shows `docs/diagrams/render-pipeline.svg`.
- `docs/materials.md` covers the material contract and the diffuse, metal and glass materials.
- `docs/geometry.md` covers the hit record, spheres, planes, triangles, texture coordinates,
  bounding boxes and the scene list.
- `docs/command-line.md` covers options, defaults, format selection, exit statuses and the progress
  line.
- `docs/image-output.md` covers channel conversion, the PPM text layout and the PNG chunk layout.
- `docs/brand/logo.svg` is the logo the root README shows.

A phase that changes the command line or a documented behavior updates the page it makes wrong and
the usage in `README.md`.

## Glossary

| Term | Meaning |
| --- | --- |
| Ray | An origin and a direction, traced once per sample and once per bounce. |
| Camera ray | The ray from the camera origin through one viewport point. |
| Sample | One ray through a jittered point inside a pixel, averaged with the pixel's other samples. |
| Bounce | One scatter of a ray at a surface, up to a limit of 50. |
| Attenuation | The colour a material multiplies the path colour by at each bounce. |
| Albedo | The attenuation colour a diffuse or metal material returns. |
| Material | How a ray scatters at a surface, or absorbs. |
| Hit record | Where a ray met a surface: ray parameter, point, normal, face side, texture coordinates and material. |
| Normal | The unit surface direction at a hit, facing against the incoming ray. |
| Front face | The side of a surface whose outward normal faces against the ray. |
| Texture coordinate | The pair `u` and `v` in 0..=1 recorded at a hit. |
| Bounding box | The axis-aligned box around an object, tested before the object's own intersection. |
| Scene list | The objects a ray is tested against, where the nearest hit wins. |
| Sky colour | The white-to-blue gradient a ray sees when it misses the scene. |
| Progress line | The standard-error line that shows the share of finished rows. |
| Seed | The generator value that fixes every random draw in a render. |
