## 1. What this repo is

A small raytracer in Rust 2021 with no dependencies beyond the standard library. It renders one
fixed scene to an image file. `src/main.rs` holds the scene, the render loop, the output and the
command line; each other module in `src/` holds one concept. There is no experiment directory.

---

## 2. Hard invariants: never violate

These bind product code. Experiments follow only the binding list in `REPO-RULES.md` §0a.

1. No dependency in `Cargo.toml`. Everything, the PNG writer included, uses only the standard
   library.
2. Rendering is reproducible: all randomness comes from `src/rng.rs`, seeded with a fixed value, so
   the same options give the same bytes.
3. Single-threaded rendering; no `unsafe`.

If a task seems to require violating an invariant, stop and ask.

---

## 3. Code style and documentation

- One concept per module in `src/`; shared types live in the module that defines the concept
  (`HitRecord` and `Hittable` in `hittable.rs`).
- Scene objects are `Box<dyn Hittable>`; materials are shared as `Rc<dyn Material>`. Avoid
  explicit lifetimes and generic parameters where a trait object works.
- Every module starts with a `//!` line saying what it holds.

---

## Test and review exceptions

- **What to test here.** Hit geometry (distance, normal, face side, texture coordinates), the
  scattering rules of each material, the PNG checksums and chunk layout, and command-line parsing.
  Rendering tests check a few pixels of a small image, never a whole image.
