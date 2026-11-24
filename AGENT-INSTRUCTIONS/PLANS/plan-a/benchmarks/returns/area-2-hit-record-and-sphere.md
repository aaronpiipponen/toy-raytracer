## Answer

`src/hittable.rs` and `src/sphere.rs` match the frozen A1/A2 contracts and requirements 7 and 8 (sphere clause) on every named point: one `HitRecord`, no `Copy`, fields `t/point/ray-facing normal/front_face/u/v/material`, the seven-argument `HitRecord::new` as the sole construction path with the only normal flip, `Rc<dyn Material>` shared and cloned per hit, the exact latitude/longitude formula, `#[allow(dead_code)]` on `u`/`v`, and the nearest-hit borrow in `HittableList::hit`. No second construction path, redundant normal check, or duplicated math exists. The only substantive weakness is that all record fields are `pub` and mutable, so the constructor's "only flip" invariant is enforceable by convention only; the rest are cosmetic.

## Findings

- **drift (latent, no live violation)** — `src/hittable.rs:12-21`: every `HitRecord` field is `pub` and thus assignable after construction, so any consumer can set `normal` or `front_face` and break the "nothing else flips a normal" rule; nothing in `src/` does today (`grep` for `.normal =` found no match). Read access is all the materials need.
- **unnecessary complexity (minor)** — `src/hittable.rs:60,64-65`: `HittableList` exposes both a public `objects` vector and `add()`, two ways to populate the scene; only `add()` is used anywhere.
- **duplication (minor, test-only)** — `src/sphere.rs:73-77,91-95,109-113`: the identical `Sphere::new(…, Rc::new(Lambertian::new(Colour::new(0.8,0.8,0.8))))` setup is repeated three times; a small local helper (as `material.rs:133-135 record_with` does) would remove it.
- **dead code** — none. The `#[allow(dead_code)]` on `u`/`v` (`src/hittable.rs:17,19`) is the contract-required marking; no other dead items in the area.
- **redundant check or record** — none in the area; the face test appears only once, in `HitRecord::new` (`src/hittable.rs:35`).
- **test bloat** — no. Three focused sphere tests (near face, back face, miss), consistent with the A2 contract; no combinatorial cases.
- **stale document** — none inside the area. (A1's frozen lines 31-32 saying "no face-side flag" are superseded by A2 and explicitly flagged there as carry-over, i.e. intentional history.)

## Out-of-scope observations

- `AGENT-INSTRUCTIONS/LOCAL/project.md` repository map lists `plane.rs`, `triangle.rs` and `aabb.rs` in `src/`, but `src/` holds only `camera, hittable, main, material, ray, rng, sphere, vec3` — either describing the plan's target or a stale map; not investigated further.
- Requirements 9-13 (planes/triangles, AABB, PNG, CLI options, progress line) are unimplemented on this branch: `src/main.rs:22-26` hardcodes 200x100/16 samples and `src/main.rs:105-127` writes PPM to stdout only. Expected mid-plan, reported for completeness.
- `src/main.rs:1` module doc still says output is PPM; requirement 11 makes PNG the default — likely a later phase's doc update.
