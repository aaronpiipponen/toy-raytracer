## Area 1 — hit record, face side, texture coordinates, sphere

**Answer.** The area is essentially clean. `HitRecord::new` is the single constructor, it sets `front_face` from `ray.direction.dot(outward_normal) < 0.0` and negates the outward normal in the one place that needs it, so the stored `normal` always opposes the ray while the shape only ever supplies a true outward normal. All three shapes (sphere, plane, triangle) build records through it and nothing re-checks or re-adjusts the normal afterwards. The sphere's `(u, v)` is the standard latitude/longitude mapping (`theta = acos(-n.y)`, `phi = atan2(-n.z, n.x) + PI`) and lands in `0..=1`, matching requirement 8. The only thing I would raise is one forward-looking comment.

**Findings**

- **stale comment (minor)** — `src/hittable.rs:12` `// The origin of the bounce rays.` describes a consumer (bounce rays) that does not exist on this branch; no material/scatter code is present and `point` is otherwise unused. A reader of this branch sees a comment referring to machinery that isn't in the tree. Low impact because the same file carries the `#[allow(dead_code)]` that explains the field is not read yet.
- Everything else is clean: no second way to build/adjust a record (only `HitRecord::new` appears; `grep` for `HitRecord {` finds no literal outside the constructor, `src/hittable.rs:33`), no redundant trailing normal check inside the shapes, no duplicated sphere math, no redundant field.

**Evidence**

- `src/hittable.rs:24-42` → one constructor; `front_face` computed once (`:27`), normal chosen/negated once (`:28-32`); doc at `:7-8` states "normal that always faces against the ray".
- `src/sphere.rs:41-45` → outward normal is `(point-center)/radius` (true outward), then latitude/longitude → `(u, v)` passed to `HitRecord::new`; no clamping/wrapping, matching `done/B1...:13-15`.
- `src/sphere.rs:89-90` → test pins `u=0.25, v=0.5` for a ray through the centre; matches the formula (`theta=PI/2`, `phi=PI/2`).
- `src/plane.rs:39`, `src/triangle.rs:48` → the other shapes use the same six-argument constructor; no shape sets `front_face`/`normal` itself, matching the full-plan design rule (`full-plan.md:29-30`).
- `src/main.rs:30` → the only product consumer of a hit reads `normal`; `u`/`v`/`front_face`/`point` are read only by tests, which is why they carry `#[allow(dead_code)]` (`src/hittable.rs:13-21`). The fields themselves are mandated by requirement 7/8 and `done/B1...:8-12`, so retaining them is required, not redundant.
- `git diff e180476..f85a183 -- src/hittable.rs src/sphere.rs` → only this area's intended changes; no leftover scaffolding.
- Requirement text checked: `docs/requirements.md:23-27` (items 7 and 8).

**Unknowns / not proven**

- A ray exactly tangent to the sphere (discriminant `== 0`) may give `dot == 0 → front_face == false`, flipping the normal inward at the measure-zero grazing point. Requirement 7's "normal points against the ray" is ambiguous at `dot == 0`; I did not treat this as a defect (hobby-project scope), but a tangent-hit test would settle whether the contract intends a tie-break.
- I did not run `cargo clippy`/`cargo test` (read-only): the `#[allow(dead_code)]` set may or may not still be required if the fields were consumed elsewhere; a clippy run would confirm nothing is unused beyond these fields.

**Out-of-scope observations (not investigated further)**

- `src/main.rs:1,67-89` writes PPM only and has no sample/gamma/progress handling, while `docs/requirements.md:34-41` (items 11-13) requires PNG, `--samples`, and a progress line. This looks like other plans' work, not geometry, but it is a visible gap between `main.rs` and the requirements.
- `src/main.rs:6-7` declares `mod plane;` with `#[allow(dead_code)]` and the scene never adds a plane (only spheres + triangle, `:36-46`); `plane.rs` is exercised only by the `hittable.rs` test. This is exactly what `done/B2...:25` specifies, so it is contract-conformant, but a later reader may wonder why a declared module is unused.
