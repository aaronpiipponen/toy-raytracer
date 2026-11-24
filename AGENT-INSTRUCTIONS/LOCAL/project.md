# Project sources of truth

- **Product overview:** `README.md` - a small dependency-free Rust raytracer that renders one fixed scene.
- **Requirements:** `docs/requirements.md` - what the program does; the section a change touches is required reading.
- **Documentation set:** `README.md` for the overview, and `docs/rendering.md`, `docs/materials.md`, `docs/geometry.md`, `docs/command-line.md` and `docs/image-output.md` for the behaviour, with the render-path diagram at `docs/diagrams/render-pipeline.svg`. `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md` owns the set and the glossary.
- **Repository map:** `src/` - `main.rs` (scene, render loop, command line and output selection), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit record, the `Hittable` trait, the scene list), `aabb.rs` (an axis-aligned bounding box and its ray test), `sphere.rs`, `plane.rs` (an infinite plane), `triangle.rs` (three vertices), `material.rs` (the `Material` trait and the diffuse, metal and glass materials), `png.rs` (the PNG writer), `rng.rs` (seeded randomness).

## Workflow slots

- Domain rules: `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md`.
- Gates: `AGENT-INSTRUCTIONS/LOCAL/gates.md`.
- Documentation: none beyond `README.md` and `docs/requirements.md`.
