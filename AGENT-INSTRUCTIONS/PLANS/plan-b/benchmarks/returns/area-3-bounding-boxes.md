## Area 3 — Bounding boxes and the scene list: CLEAN

I found no defect, duplication, dead code, drift from the frozen B3 contract, or unnecessary complexity in this area. Details of what I checked and why it is correct are in Evidence; the only things worth noting are coverage observations, not findings.

**Evidence**
- `src/aabb.rs:19-46` → standard slab test: per-axis `near`/`far` from `min`/`max`, swap when inverted, `t_enter = max`, `t_exit = min`, early false on `t_enter > t_exit`. Matches the frozen contract's inclusive `t_min..t_max` wording.
- `src/aabb.rs:27-32` → a `direction == 0.0` axis is handled as a pure slab-containment test (`origin < min || origin > max`), exactly the contract's "misses only when its origin lies outside that slab".
- `src/aabb.rs:7-10` → `Aabb` is a plain `pub min/max` value; no stored/cached/refit box, and no second culling path exists (grep finds `Aabb` used only in `aabb.rs`, `hittable.rs`, `sphere.rs`, `triangle.rs`).
- `src/hittable.rs:50-52` → `bounding_box` default returns `None`; `Plane` (`src/plane.rs`) and `HittableList` do not override it, so the plane is always tested — exactly the contract's rule that only `Sphere` and `Triangle` override.
- `src/hittable.rs:70-79` → `limit` is computed once per object and passed to both `bounds.hit(ray, t_min, limit)` and `object.hit(ray, t_min, limit)`, so the box and the object share the same limit; a boxless object (`None`) falls through to the object test. No redundant box re-check after a hit.
- `src/sphere.rs:48-51` → `center ± (radius, radius, radius)` on every axis, contains the sphere, uses the same `self.radius` as `Sphere::hit`.
- `src/triangle.rs:51-63` → component-wise min/max of the three vertices; contains the triangle. The degenerate (zero-thickness) slab is handled correctly by the slab test, including the all-`z = -1` scene triangle.
- `src/main.rs:3` → `mod aabb;` is the first module.
- `git diff --stat e180476..f85a183 -- docs/ README.md AGENT-INSTRUCTIONS/LOCAL/project.md` → empty; item 10 of `docs/requirements.md` and `project.md`'s map entry (`aabb.rs`) are consistent with the code.
- Test count `12` matches the contract's "12 passing tests"; the three named proof tests exist (`src/aabb.rs:53`, `src/hittable.rs:105`, `src/triangle.rs:88`) and the list test's `Unreachable` panicking `hit` genuinely proves the box-miss skip.

**Coverage note (not a defect against the contract)**
- `src/aabb.rs:53-60` tests only a through-ray and a beside-ray. The frozen contract's explicitly specified zero-direction branch (`src/aabb.rs:27-32`) and the `t`-range clipping are unexercised. The contract names only this one test, so this is not drift; if extra proof is ever wanted, a ray parallel to and/or starting inside a slab would cover it. `Sphere::bounding_box` also has no direct test, but the contract does not require one.

**Out-of-scope observations**
- `src/main.rs:1` and `src/main.rs:86-89` → the file documents and writes PPM only, while `docs/requirements.md:11-13` require PNG/format/CLI/progress. This is the output plan's scope, not geometry; flagging only so it is not mistaken for a plan-b gap.
- `src/hittable.rs:13,16,19,21` → `point`, `front_face`, `u`, `v` carry `#[allow(dead_code)]`; unused in the current product path until the materials phase. Outside this area, mentioned for completeness.

**Unknowns** — none material; I did not run `cargo test`/`clippy` (read-only brief), so the "12 passing tests / clippy clean" claim is taken from the frozen contract, not re-verified.
