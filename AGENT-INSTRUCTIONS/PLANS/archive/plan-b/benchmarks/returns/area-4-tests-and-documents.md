## Answer

Area 4 is essentially clean. The seven plan-added tests split 2 / 2 / 3 across phases B1–B3, all within REPO-RULES §3a's cap of 3 per phase, with no fake product behaviour, no wording/private-helper assertions and no quota packing; the 12-test total matches the B3 receipt. `README.md` and `docs/requirements.md` items 7–10 accurately describe what the code does now. The one real defect is a stale repository map: `AGENT-INSTRUCTIONS/LOCAL/project.md:5` lists `src/material.rs`, which does not exist on this branch (it belongs to the materials plan).

## Findings

**1. `drift` (stale repository map) — `AGENT-INSTRUCTIONS/LOCAL/project.md:5`**
The map lists `material.rs` (the `Material` trait and the diffuse and metal materials), but no such module exists in this worktree. `git ls-tree`/`ls src/` show only `aabb, camera, hittable, main, plane, ray, rng, sphere, triangle, vec3`. A reader of the map is told a module exists that the branch cannot build or open.
Nuance: the file is a symlink to the shared `run004/AGENT-INSTRUCTIONS/LOCAL/project.md`, and `../plan-a/src/material.rs` does exist — so the map may be a cross-plan target map rather than an error. It is still wrong for this tree and should be verified against the intended contract. No module is omitted.

**2. `nit` — `docs/requirements.md:29`** "Every object reports an axis-aligned bounding box" is immediately qualified by "An infinite plane has no box"; `HittableList` also takes the default `None` and is not mentioned. The sentence is slightly imprecise but item 10's operative behaviour (box report, list tests box first, boxless plane always tested) matches `src/hittable.rs:50-53,72-76` and `src/plane.rs` (no override). Not a defect on its own.

**Clean checks (explicit):**
- **Test budget §3a:** B1 added 2 (`src/sphere.rs:71,83`), B2 added 2 (`src/plane.rs:48`, `src/triangle.rs:71`), B3 added 3 (`src/aabb.rs:54`, `src/hittable.rs:106`, `src/triangle.rs:89`) — each ≤ 3. No phase split, no quota packing. Total 12.
- **Test focus:** tests assert public hit geometry (distance, normal, front face, `(u,v)`) and the requirement-10 box contract; the one double (`Unreachable`, `src/hittable.rs:90-103`) proves box-skipping ordering by panicking if `hit` is called — legitimate, not re-implemented product behaviour.
- **No wording/private-helper assertions:** exact values are geometric contracts, not strings. `Aabb::hit`, `bounding_box`, `render` are the tested behaviours.
- **Delete-with-code:** no test was orphaned or deleted; the only removed lines in the range are the `Vec3::cross` `#[allow(dead_code)]` scaffolding (`src/vec3.rs:21`, now used by `plane.rs`/`triangle.rs`).
- **Requirements 7–10 and README:** accurate against the code (item 7 = `HitRecord::new` `front_face`/facing normal `src/hittable.rs:26-41`; item 8 = sphere lat/long `src/sphere.rs:41-44`, plane modulo `src/plane.rs:37-38`, triangle barycentric `src/triangle.rs:48`; item 9 = plane/triangle modules + triangle in `src/main.rs:40-44`; item 10 as above). README's "one fixed scene / `cargo run > image.ppm`" matches current PPM-to-stdout behaviour.
- **No plan references** in `README.md`, `docs/`, or `src/` (grep for plan ids / `PLANS/` returned nothing).

## Out-of-scope observations

- **`dead code` — `src/main.rs:9-10`, `src/rng.rs`:** `Rng`/`next_f64` are used by no product code (only the rng test and comments); `mod rng` carries `#[allow(dead_code)]`. Scaffolding for the materials/output plans per requirement 6, but presently dead.
- **`dead code` — `src/hittable.rs:12-14`:** `HitRecord::point` ("origin of the bounce rays") has no reader in product or tests; kept behind `#[allow(dead_code)]` for the materials plan.
- **`dead code` — `src/main.rs:6-7`:** `mod plane` carries `#[allow(dead_code)]` because `Plane` is constructed only in the hittable test, never in `scene()`. Frozen into the B2 contract, so intended.
- **`test bloat` (pre-existing, low) — `src/main.rs:95-106`:** both rendering tests call `render(WIDTH, HEIGHT)` (the full 200×100 image) and each renders it again; the domain test exception asks for "a few pixels of a small image". Not plan-introduced and cheap, so informational.
