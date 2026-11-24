# Project sources of truth

- **Product overview:** `README.md` - a small dependency-free Rust raytracer that renders one fixed scene.
- **Requirements:** `docs/requirements.md` - what the finished program does; the section a change touches is required reading.
- **Repository map:** `src/` - `main.rs` (scene, render loop, output, command line), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit record, the `Hittable` trait, the scene list), `aabb.rs` (an axis-aligned bounding box and its ray test), `sphere.rs`, `plane.rs` (an infinite plane), `triangle.rs` (three vertices), `material.rs` (the `Material` trait and the diffuse and metal materials), `rng.rs` (seeded randomness).

## Workflow slots

- Domain rules: `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md`.
- Gates: `AGENT-INSTRUCTIONS/LOCAL/gates.md`.
- Documentation: none beyond `README.md` and `docs/requirements.md`.
