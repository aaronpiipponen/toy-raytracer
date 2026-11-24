# Project sources of truth

- **Product overview:** `README.md` - a small dependency-free Rust raytracer that renders one fixed scene.
- **Requirements:** `docs/requirements.md` - what the finished program does; the section a change touches is required reading.
- **Repository map:** `src/` - `main.rs` (scene, render loop, output, command line), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit record, the `Hittable` trait, the scene list), `material.rs` (the `Material` trait and the diffuse, metal and glass materials), `sphere.rs`, `plane.rs` (infinite plane), `triangle.rs` (triangle), `aabb.rs` (axis-aligned box), `png.rs` (PNG writer), `rng.rs` (seeded randomness).

## Workflow slots

- Domain rules: `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md`.
- Gates: `AGENT-INSTRUCTIONS/LOCAL/gates.md`.
- Documentation: none beyond `README.md` and `docs/requirements.md`.
