# plan-b closing survey - saved area returns (2026-11-24)

Branch `plan-b`, worktree `/home/opc/projects/toy-raytracer-runs/run003/plan-b`, survey base
`e180476c23edf84bdd26d612cc5ab3ca32eeaf1b`, head `c25007c1281b59e4e24e94ec473988d8105ebbea`.
Changed files: `src/aabb.rs`, `src/hittable.rs`, `src/main.rs`, `src/plane.rs`, `src/sphere.rs`,
`src/triangle.rs`, `src/vec3.rs`. These are the six verbatim read-only `ops-scout` area returns saved
as the finding set for `benchmarks/2026-11-24-closing-survey.md`. Nothing is edited or added here.

## Area 1 - Hit record and face side

**Answer**
Area 1 is clean on construction, face-side math and derives: the only `HitRecord` struct literal is inside `HitRecord::new` at `src/hittable.rs:34`, and all three shapes call it (`sphere.rs:44`, `plane.rs:44-51`, `triangle.rs:47`). `front_face` is computed as `ray.direction.dot(outward_normal) < 0.0` and the stored `normal` is the outward normal only when `front_face`, otherwise its negation, exactly matching the frozen contract; the derives are `Clone, Copy, Debug` as required. However the four `#[allow(dead_code)]` attributes are not backed by any product consumer on this branch: `point` is never read anywhere (not even in tests), and `front_face`, `u` and `v` are read only by `#[cfg(test)]` assertions, so the `point` allow is unexplained dead code and the other three are test-only on this branch, kept alive only by the contract's forward-looking field list. No comment in this area mentions plan history.

**Evidence**
- `src/hittable.rs:34-41` → the sole struct literal, built by `new`.
- `rg "HitRecord"` over `src/` → 15 matches, only calls are `sphere.rs:44`, `plane.rs:44`, `triangle.rs:47`; no other `HitRecord { … }`.
- `src/hittable.rs:28` → `let front_face = ray.direction.dot(outward_normal) < 0.0;`
- `src/hittable.rs:29-33` → `normal` is `outward_normal` when `front_face`, else `-outward_normal`.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1r-shared-hit-record-shape.md:5-13` → requires those fields, `Clone, Copy, Debug`, `new` as the only construction, and `front_face` from the dot-product sign with a flipped normal.
- `src/hittable.rs:9` → `#[derive(Clone, Copy, Debug)]`.
- `src/sphere.rs:67,78,89-90`, `src/plane.rs:70,72-73`, `src/triangle.rs:82,84-85` → `front_face`/`u`/`v` read only in test modules.
- `rg "front_face|\.point|\.u\b|\.v\b"` over `src/` → no reads of `hit.point` and no `point` field access at all; `hit.front_face`/`hit.u`/`hit.v` appear only in the listed test lines.
- `src/hittable.rs:12-21` → `point`'s comment says "The origin of the bounce rays", but `src/main.rs:28-35` colours by `hit.normal` and `src/hittable.rs:66-81` copies whole records, so no bounce-ray consumer exists.
- `src/hittable.rs:7-8` and `25-26` → doc comments describe behaviour only, no plan/phase references.
- `src/sphere.rs:40-44` → `(u, v)` computed from `outward_normal` and passed with the outward normal, matching the contract's sphere clause.
- `src/sphere.rs:65-67,77-79` → near-surface hit asserts front face with normal `(0,0,1)`; inside hit asserts back face with normal `(0,0,-1)`, consistent with the sign convention.

**Unknowns**
- Whether the four `dead_code` allows are deliberate transitional scaffolding for the release merge (the contract says the release merge consumes `point`/`front_face`/`u`/`v`) versus incidental suppression: the area carries no consumer evidence either way, and reading later-phase plans was out of scope.
- Whether the compiler would warn without the allows (test-only field reads typically do not count for `dead_code`) — could not confirm without a build, which is forbidden here.

**Out-of-scope observations**
- `src/main.rs:8-10` also carries `#[allow(dead_code)] mod rng;` with a comment about seeded randomness, suggesting the same kind of forward-looking suppression outside area 1.

## Area 2 - Texture coordinates

**Answer**
All three mappings match the contract and stay inside the contracted closed range in exact arithmetic, so I found no mapping that leaves it, no coordinate taken from the ray-facing normal and no coordinate recomputed after the fact. Spheres map latitude/longitude from the outward normal `(point - center) / radius`, giving `u = phi/(2π)` with `phi = atan2(-n.z, n.x) + π` and `v = theta/π` with `theta = acos(-n.y)`; `u` lies in `(0, 1]` and `v` in `[0, 1]`, and because the normal is the outward one a front-face and a back-face hit of the same point record identical `(u, v)`. Planes take the two hit-point axes not used by the dominant `|normal|` component and apply `rem_euclid(1.0)`, which yields `[0, 1)`; the tests confirm `(1.25, -0.25)` → `(0.25, 0.75)`. Triangles record the Möller–Trumbore weights unchanged, so the miss rejects keep `0 ≤ u ≤ 1`, `v ≥ 0`, `u + v ≤ 1`. Every coordinate is computed before, and passed into, the single `HitRecord::new` construction path; `main.rs` currently reads only `hit.normal`, so nothing downstream recomputes `u`/`v`.

**Evidence**
- `src/sphere.rs:39-43` — outward normal from the surface point, then `theta = (-n.y).acos()`, `phi = (-n.z).atan2(n.x) + PI`, `u = phi/(2π)`, `v = theta/π`; no clamp or wrap (as the contract requires).
- `src/sphere.rs:44` — those `u`, `v` and the outward normal go straight into `HitRecord::new`.
- `src/hittable.rs:28-29` — `front_face` is derived from the ray direction and only `normal` is flipped; `u`/`v` are untouched, so they cannot come from the ray-facing normal. `src/hittable.rs:19-21` — `u` and `v` are record fields.
- `src/plane.rs:35-43` — dominant `|normal|` axis selects `(y,z)`, `(x,z)` or `(x,y)`; `src/plane.rs:49-50` — `rem_euclid(1.0)` on both.
- `src/plane.rs:65-74` (test) — hit point `(1.25, -0.5, -0.25)` on the `+y` plane gives `u = 0.25`, `v = 0.75` after wrapping.
- `src/triangle.rs:33-41` — `u = inv_det * s·h` is range-checked to `0.0..=1.0`, and `v = inv_det * d·q` is rejected for `v < 0` or `u + v > 1`, so the recorded weights cannot leave the range.
- `src/triangle.rs:46-47` — winding-derived outward normal `edge1.cross(edge2).unit()`; `u`, `v` recorded unchanged.
- `src/triangle.rs:71-86` (test) — barycentric `u = v = 0.25` recorded.
- `src/sphere.rs:83-91` (test) — `(0,0,1)` normal gives `u = 0.25`, `v = 0.5`.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1r-shared-hit-record-shape.md` — contract: sphere `(u,v)` from the outward normal, exact `theta`/`phi` formulas, "without clamping or wrapping"; `done/B2-planes-and-triangles.md` — plane dominant-axis `rem_euclid`, triangle barycentric weights, both two-sided via `HitRecord::new`.
- `docs/requirements.md:25-27` (item 8) — `(u, v)` in `0..=1` by those three schemes.
- `src/main.rs:30-33` — `ray_colour` uses only `hit.normal`; `grep '\.u\b|\.v\b'` in `src/main.rs` finds only the camera's pixel coordinates (`src/main.rs:59-61`), so no consumer recomputes hit `u`/`v`.
- `src/vec3.rs:33-35` — `unit()` is `self / self.length()`, no clamp helper exists.

**Unknowns**
- The sphere's `acos` is unclamped (`src/sphere.rs:40`) on a normal whose unit length is only floating-point exact; if a computed `|n.y|` overshoots 1 by an ulp (e.g. a ray aimed exactly at a pole), `acos` yields NaN and `v` becomes NaN rather than a value outside `0..=1`. I did not run anything to observe whether that overshoot actually occurs, and the contract explicitly forbids clamping, so this is a numerical boundary rather than a demonstrated violation.
- Sphere `u` never reaches exactly 0 (range `(0,1]`); this is inside item 8's closed `0..=1` and matches the contract, so I did not treat it as a defect.

**Out-of-scope observations**
- `u` and `v` are `#[allow(dead_code)]` (`src/hittable.rs:18-21`) because no production material reads them yet; this matches the B1r contract and is expected to become live in the release merge, not a fault here.

## Area 3 - Plane and triangle

**Answer**
Area 3 matches the frozen B2 contract at head c25007c on every checked point: `Plane::new` normalises with `normal.unit()`, `Plane::hit` uses the `1e-9` parallel epsilon and strict `t_min < t < t_max`, and maps the hit to the two non-dominant axes with `rem_euclid(1.0)`; `Triangle::hit` is a textbook Möller–Trumbore with the `1e-8` determinant epsilon, `u` on edge-to-`b`, `v` on edge-to-`c`, and barycentric rejection that makes a degenerate triangle never hit. Both shapes build every record through the shared `HitRecord::new`, so both are two-sided; neither carries a material, an `Rc` or a one-sided switch, and there is no second hit path. The scene holds exactly the contracted sphere, plane and triangle vertices with normal `(0,0,1)`, `Vec3::cross` no longer carries `#[allow(dead_code)]` or its purpose comment, and there are no dead helpers or history/plan comments in these files. There is no drift: the only contract lines that read as "future" (`bounding_box` and the triangle box test) have since landed with B3, exactly as B2 anticipated.

**Evidence**
- `src/plane.rs:19` → `normal: normal.unit()`.
- `src/plane.rs:27` → `if denominator.abs() < 1e-9 { return None; }`.
- `src/plane.rs:31` → `if t <= t_min || t >= t_max { return None; }`.
- `src/plane.rs:35-43` → 3-way dominant-axis choice over the non-dominant axes, then `rem_euclid(1.0)` at `50-51`; matches contract lines 8-9.
- `src/plane.rs:44-51` → record built via `HitRecord::new(ray, t, point, self.normal, u, v)`.
- `src/plane.rs:54-57` → `bounding_box` returns `None` (B3 work, per `done/B3-bounding-boxes.md:16-17`).
- `src/triangle.rs:27-29` → `determinant = edge1.dot(h)`; `if determinant.abs() < 1e-8 { return None; }`.
- `src/triangle.rs:26,32-38` → `h = dir.cross(edge2)`, `u = inv * s.dot(h)` weights edge1 to `b`; matches contract line 11-12.
- `src/triangle.rs:34` → `(0.0..=1.0).contains(&u)`; `src/triangle.rs:39` → `v < 0.0 || u + v > 1.0`, combined bounds u≥0, v≥0, u+v≤1.
- `src/triangle.rs:46-47` → `outward_normal = edge1.cross(edge2).unit()`, record via `HitRecord::new` (two-sided).
- Degenerate rejection: for collinear `edge1,edge2` the determinant is 0 → epsilon path rejects (`src/triangle.rs:28-29`).
- `src/main.rs:39-48` → sphere `(0,0,-1),0.5`, plane `(0,-0.5,0),(0,1,0)`, triangle `(-2,-0.5,-2),(2,-0.5,-2),(0,1.5,-2)`; `edge1×edge2=(0,0,8)`→unit `(0,0,1)`.
- `git diff e180476 c25007c -- src/vec3.rs` → removed `#[allow(dead_code)]` and the "For the triangle normals the geometry plan adds." comment from `cross`.
- `grep -nE 'material|Rc' src/plane.rs src/triangle.rs` → none.
- `grep 'fn '` on both files → `Plane::new`, `hit`, `bounding_box`; `Triangle::new`, `hit`, `bounding_box`; no other helpers.
- `grep -niE 'plan|B2|B3|yet|geometry' src/` → no history/plan comments in area-3 files (only identifiers containing "plane").
- `src/plane.rs:64-74` test name/assertions (t=0.5, front face, normal `(0,1,0)`, u=0.25, v=0.75) match contract lines 41-43.
- `src/triangle.rs:70-86` and `88-97` test names/assertions (t=1, front face, normal `(0,0,1)`, u=v=0.25; miss beside) match contract lines 44-45.
- `src/triangle.rs:99-109` third test `the_triangle_box_holds_its_three_vertices` is the B3 test (`done/B3-bounding-boxes.md:24`), not a B2 drift.
- `git log --oneline e180476..c25007c` → B2 is `9afa2db`; head `c25007c` is B3 "Skip objects whose bounding box the ray misses", explaining the pre-B3 wording at `B2-planes-and-triangles.md:26`.

**Unknowns**
- Whether a build/lint would warn on anything is not established: I was fenced from compiling or running clippy. Static reads show every item in `plane.rs`/`triangle.rs` used (`Plane::new`, `Triangle::new` in `src/main.rs:40,44` and tests; `cross` used at `src/triangle.rs:26,37,46`), so no obvious dead helper remains.
- The B2 contract's line 26 ("`bounding_box` does not exist yet") describes the B2 freeze point; head has it via B3. I treated that as anticipated subsequent work, not drift, but cannot inspect the B2-only tree without a checkout.

**Out-of-scope observations**
- `src/hittable.rs:10-22` still marks `point`, `front_face`, `u`, `v` with `#[allow(dead_code)]` even though `u`/`v` and `front_face` are now read by tests; harmless, but the allow attributes are now partly unnecessary (not area 3, and the B2 contract's "HitRecord unchanged" wording covers the shape).
- `Plane` and `Triangle` expose `pub` fields (`src/plane.rs:9-12`, `src/triangle.rs:9-13`), so a caller could build a `Plane` struct literal with a non-unit `normal`, bypassing `Plane::new`'s normalisation. This conforms to the frozen contract (which specifies `pub` fields), so it is a latent interface risk rather than a drift; only `new` is used in-tree (`src/main.rs:40`, `src/hittable.rs:99`, `src/plane.rs:66`).
- `src/hittable.rs:66-81` (HittableList box skip) and `src/aabb.rs` are B3 work outside area 3 and were not judged.

## Area 4 - Bounding boxes

**Answer**
Area 4 matches the frozen B3 contract: `Aabb::new` orders each axis (`min`/`max`) at `src/aabb.rs:14-19`, and `Aabb::hit` is a plain three-slab rejection over the caller's `t_min..t_max` that treats an exactly-zero direction component by rejecting only when the origin is outside that slab (`src/aabb.rs:30-44`). `Hittable` carries a required (non-default) `bounding_box` (`src/hittable.rs:49-50`), `Sphere` returns the exact radius box (`src/sphere.rs:48-51`), `Triangle` returns the unpadded per-axis vertex bounds (`src/triangle.rs:51-62`), and `Plane` and `HittableList` return `None` (`src/plane.rs:55-57`, `src/hittable.rs:84-86`), so both remain unconditionally tested. `HittableList::hit` tests a present box once over the full `t_min..t_max` before computing the running `limit = nearest.map_or(t_max, |hit| hit.t)` (`src/hittable.rs:69-78`); because every real hit point lies inside the object's box, a failed box test can only skip an object that has no hit, so the chosen record is unchanged. There is no padding, shrink, merge, ordering, hierarchy, cached box, or second hit path anywhere (`grep bounding_box|Aabb src/` shows only the trait, the three shapes, and the list test).

Coverage is thinner than the contract's prose implies: the tests exercise one crossing ray, one spatially-missed ray (which also exercises the parallel-outside slab case), and the list still hitting a boxless plane, but they do **not** test corner ordering (the test builds an already-ordered box), `t_min..t_max` rejection (nothing beyond the range or behind the ray), the sphere's box, or a direct `None` assertion for `Plane`/`HittableList`. The byte-identical-image claim is not currently evidenced in the tree or `/tmp`.

**Evidence**
- `src/aabb.rs:14-19` → per-axis `min`/`max` ordering in `new`.
- `src/aabb.rs:22-44` → mutating `t_min`/`t_max` slab clamp, `t_max < t_min` rejection, zero-direction inside/outside handling.
- `src/aabb.rs:55-61` → one crossing ray and one ray at `x = 2`, `direction.x = 0` (parallel-outside case).
- `src/hittable.rs:49-50` → `bounding_box` is a required trait method (no default).
- `src/hittable.rs:69-78` → box tested over `t_min..t_max` before `let limit = nearest.map_or(t_max, |hit| hit.t)`.
- `src/hittable.rs:96-108` → list test: sphere box missed at `x = 5`, boxless plane still hit at `t = 0.5`.
- `src/sphere.rs:48-51` → `center ± (r,r,r)` box.
- `src/plane.rs:55-57` → `None` with infinite-plane rationale.
- `src/triangle.rs:51-62` → per-axis vertex min/max, no padding.
- `src/triangle.rs:99-109` → triangle box exactly equals the three vertex bounds.
- `grep -c '#\[test\]' src/*.rs` → 13 test functions total (aabb 1, hittable 1, plane 1, sphere 4, triangle 3, main 2, rng 1), matching the contract's suite count.
- `docs/requirements.md:29-30` → item 10 states the box test and the plane's missing box.
- `git show --stat HEAD` → only `aabb.rs`, `hittable.rs`, `main.rs`, `plane.rs`, `sphere.rs`, `triangle.rs` touched in c25007c.
- `grep -rn "bounding_box\|Aabb" src/` → no cached box field, no nested/hierarchy use, no second box-test path.
- `ls /tmp/opencode/b3-*.ppm` → no such files; `cmp` of any surviving b3 render could not be run.
- `cmp /tmp/opencode/b3scratch.ppm /tmp/opencode/render-before.ppm` → differ at byte 32405 (`render-before` is dated Nov 23 22:35, not the B2 render from 9afa2db).

**Unknowns**
- Whether the required rendered-image gate actually produced a byte-identical PPM: its `b3-before.ppm`/`b3-after.ppm` artifacts are absent, and `b3scratch.ppm` cannot be identified as the B3 result or compared (no B2 render retained). The contract's "`cmp` clean" statement is therefore unverified from current evidence. Reasoning says the image cannot change (box test is conservative and hit order is untouched), but that is not a recorded comparison.
- Exact-float boundary behavior: the box test uses inclusive slab endpoints with no epsilon, so a true triangle hit whose `ray.at(t)` rounds marginally outside the vertices' f64 hull could in principle be rejected. No test or artifact demonstrates or rules this out; it is a general floating-point edge, not an observed drop.
- Whether any corner-order test exists outside this worktree (e.g. a later phase): none found in `src/` at HEAD.

**Out-of-scope observations**
- `AGENT-INSTRUCTIONS/LOCAL/project.md`'s repository map still omits `aabb.rs`, `plane.rs` and `triangle.rs`; the B3 contract records this as release-owned (`AGENT-INSTRUCTIONS/PLANS/plan-b/done/B3-bounding-boxes.md`, "Documentation state").
- `src/main.rs:37-52` replaces the base scene's huge ground sphere with `Plane` + `Triangle`, so any image comparison against survey base `e180476` would legitimately differ; only a comparison against 9afa2db is meaningful for B3's byte-identical gate.
- `docs/requirements.md:29` says "Every object reports an axis-aligned bounding box" while line 30 carves out the infinite plane; the pair is accurate in combination but reads as internally tense.

## Area 5 - Scene and render output

**Answer**
Area 5 is clean and matches the frozen B2 contract. `scene()` holds the three objects in the contract's order with exactly the specified values (sphere `(0,0,-1)` r=0.5; plane through `(0,-0.5,0)` normal `(0,1,0)`; triangle `(-2,-0.5,-2)`, `(2,-0.5,-2)`, `(0,1.5,-2)`). Render order is top-to-bottom (`v = row/(height-1)` with rows iterated in reverse) and the camera aspect is `WIDTH/HEIGHT = 2.0`, with `Camera::new` taking that ratio directly; both render tests still describe their pixels correctly. There is exactly one output path: `write_ppm` writing the P3 header and pixel lines to the caller's writer, called once from `main` with a locked `io::stdout()`; nothing prints to stdout or stderr beyond those PPM bytes. No randomness is generated anywhere in the render path (the `Rng` module is dead code awaiting scattering), and there is no `rand` dependency, so any future random value must come from the seeded generator; no material, `Rc`, command-line parsing, PNG or progress output exists on the branch.

**Evidence**
- `src/main.rs:37-50` → scene sphere/plane/triangle values and order match `B2-planes-and-triangles.md:18-22` exactly.
- `src/main.rs:25-26` → `WIDTH=200`, `HEIGHT=100`; `src/main.rs:55` → `Camera::new(width as f64 / height as f64)` (aspect 2.0); `src/camera.rs:14-21` → viewport height 2, width `aspect*height`, origin at 0 looking −z.
- `src/main.rs:57-61` → `for row in (0..height).rev()` and `v = row/(height-1)`, so the first pushed pixel is the top row; `src/main.rs:95-110` → tests unchanged and consistent (`pixels[0]` is the top row).
- `src/main.rs:71-88` → single `write_ppm` emitting `P3` header plus one line per pixel; `src/main.rs:90-93` → `main` calls it once on `io::stdout().lock()` and returns its `io::Result`.
- `grep File::create|write_all|stdout|stderr|\.ppm|\.png|env::args|clap src/` → only `src/main.rs:92` stdout use; no `println!`/`eprintln!` anywhere.
- `Cargo.toml` → `[dependencies]` empty; `grep Rng|next_f64|next_u64 src/ | grep -v src/rng.rs` → no matches, so no random value is consumed and no alternate RNG exists.
- `git log e180476..c25007c` / `git status --short` → seven commits, working tree clean at c25007c; `git diff ... -- src/main.rs` shows only scene/plane/triangle and rng-comment changes.
- `src/vec3.rs:21` → `cross` now has no `#[allow(dead_code)]` and no old purpose comment (matches `B2:23-24`).

**Unknowns**
- Whether the rendered image itself is correct (sky gradient, plane as flat bottom half, triangle placement) was not verified: I am read-only and did not build or run the binary, so the `B2:49-50` gate claim rests on the contract document only.
- Nothing was found to be missing; every requested check had direct evidence.

**Out-of-scope observations**
- `src/main.rs:8` comment "Seeded randomness for scattering and sampling" and the `#[allow(dead_code)]` at `src/main.rs:9` describe `rng` as used by scattering, which does not exist yet; this is forward-looking, not false, but worth noting if a future phase changes the purpose.
- `src/main.rs:30` uses a hardcoded `0.001` hit epsilon inline; acceptable for this project but not named as a constant.

## Area 6 - Tests and documents

**Answer**
The suite is exactly 13 test functions and within every budget: base `e180476` had 5 (`main.rs` 2, `sphere.rs` 2, `rng.rs` 1), B1r added 2 sphere tests, B2 added 3 (plane 1, triangle 2) and B3 added 3 (`aabb.rs` 1, `hittable.rs` 1, triangle 1), each phase ≤3, with no duplicated behaviour, no private-helper or exact-wording assertions, and no plan id in any test name. Requirements items 7 to 10 match this branch's code and items 1 to 6 and 11 to 14 are merged-product text whose absence is expected branch state, `README.md` matches the branch's PPM-on-stdout behaviour, the repository-map omission is recorded in `docs-pass.md:5-12`, and the rendered-image gate still applies through the `gates.md:39` fallback because this branch has no command line. Two document problems remain: `docs/requirements.md:19` names the module `src/rng.rs` in a product page, and the release record is stale and incomplete — plan-b's branch already removed the plan-quoting comments at `main.rs:8` (commit `281dda8`) and `hittable.rs:12` (commit `faac698`), yet `plan-b/docs-pass.md` lists only the repository map, while `plan-c/docs-pass.md:26-34` still assigns both comments to plan-a's release and only acknowledges the `vec3.rs` item as resolved by plan-b.

**Evidence**
- `grep -rn '#\[test\]' src` → 13: `aabb` 1, `hittable` 1, `main` 2, `plane` 1, `rng` 1, `sphere` 4, `triangle` 3; `src/aabb.rs:54`, `src/hittable.rs:95`, `src/main.rs:99,106`, `src/plane.rs:64`, `src/rng.rs:29`, `src/sphere.rs:58,70,82,93`, `src/triangle.rs:70,88,99`.
- `git show e180476:src/main.rs | grep -c '#\[test\]'` → 2; `git show e180476:src/sphere.rs` → `a_ray_through_the_center_hits_the_near_surface` and `a_ray_beside_the_sphere_misses`; base per-file loop → `aabb:0 hittable:0 plane:0 triangle:0` plus `rng:1`.
- `git show faac698` → adds `a_ray_from_inside_the_sphere_hits_the_back_face` and `the_near_surface_has_texture_coordinates_from_latitude_and_longitude` (2, B1r).
- `git show 9afa2db` → adds `src/plane.rs:65` and `src/triangle.rs:71,89` (3, B2).
- `git show c25007c` → adds `src/aabb.rs:55`, `src/hittable.rs:96`, `src/triangle.rs:100` (3, B3).
- `src/sphere.rs:40-43`, `src/plane.rs:35-51`, `src/triangle.rs:47` → latitude/longitude, dominant-axis position with `rem_euclid(1.0)`, barycentric coordinates: requirements item 8 (`docs/requirements.md:25-27`) accurate; `src/hittable.rs:27-33` → face side and against-ray normal (item 7); `src/hittable.rs:69-74`, `src/plane.rs:55-57` → box tested before object, plane always tested (item 10).
- `git diff e180476..c25007c -- docs/requirements.md README.md` → empty; `src/main.rs:1` and `README.md:6` → PPM to standard output, so `cargo run > image.ppm` is true for this branch.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/docs-pass.md:7-12` → records adding `aabb.rs`, `plane.rs`, `triangle.rs` and plan-c's `png.rs` half at the release; `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md:21-24` → confirms the `png.rs` half.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md:30,36,39` → rendered-image gate triggers on render-loop changes and its fallback PPM command applies until the command line lands.
- `git show e180476:docs/requirements.md` line 6 → item 6 already read "All randomness comes from the seeded generator in `src/rng.rs`"; unchanged on this branch.
- `git show 281dda8` → `main.rs` comment changed from "…the plans add." to "Seeded randomness for scattering and sampling."; `git log -L 10,16:src/hittable.rs` → `faac698` changed "The origin of the bounce rays the materials plan adds." to "The origin of the bounce rays."
- `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md:26-34` → still calls those two comments owned by plan-a's release; `:36-40` → says plan-b's branch already removes the `vec3.rs` comment and allowance (`src/vec3.rs:21` confirms no `#[allow(dead_code)]`, and `src/triangle.rs:26,37,46` call `cross`).

**Unknowns**
- I did not run the fast gate, so build/test/clippy/fmt pass status and the byte-identical render claim are unverified (read-only). Evidence needed: `cargo test`/`cargo build` output and a before/after PPM diff.
- Whether the two stale `plan-c/docs-pass.md` entries count as a "missing release record" for plan-b or are simply plan-c's to clear at the shared release is a workflow judgement; plan-c's own text says the release reads and deletes resolved items, but it does not mention the plan-b branch rewrite of `main.rs`/`hittable.rs`.
- The merged-product requirements items cannot be checked against this worktree since materials and PNG/CLI code are absent here by design.

**Out-of-scope observations**
- `src/main.rs:95-110` tests call the private `render` helper and render the full 200x100 image while asserting two pixels; domain guidance prefers "a few pixels of a small image", and REPO-RULES §3a prefers no private-helper tests. Both tests predate this plan (`git show e180476:src/main.rs`), so this is branch history, not new drift.
- All tests sit inline in `#[cfg(test)] mod tests` rather than "sub-directories mirroring the product packages" (REPO-RULES §3a Layout); this is the repository's existing convention, not introduced by plan-b.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/` does not exist yet; the Z1 note is pending phase work, not a release record.
