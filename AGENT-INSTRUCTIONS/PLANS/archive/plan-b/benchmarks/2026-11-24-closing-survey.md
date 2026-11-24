# plan-b closing survey - 2026-11-24

Branch `plan-b`, worktree `/home/opc/projects/toy-raytracer-runs/run003/plan-b`, survey base
`e180476c23edf84bdd26d612cc5ab3ca32eeaf1b`, head `c25007c1281b59e4e24e94ec473988d8105ebbea`.

**Method.** Six independent read-only `ops-scout` sessions each surveyed one area, all started before
any return was read; their verbatim returns are saved in
`AGENT-INSTRUCTIONS/PLANS/plan-b/survey-returns-2026-11-24.md`. One fresh independent read-only
`ops-scout` then cross-checked every claim against the tree and checked coverage. The orchestrator
wrote this note from the saved returns and the cross-check. **Nothing was fixed in this phase**, and
no product code, test, `docs/` page, `Cargo.toml`, `AGENT-INSTRUCTIONS/LOCAL/` file or other plan's
file was changed.

## Coverage

| Changed path | Area(s) |
|---|---|
| `src/aabb.rs` | 4 |
| `src/hittable.rs` | 1, 4 |
| `src/main.rs` | 3, 5 |
| `src/plane.rs` | 2, 3 |
| `src/sphere.rs` | 1, 2 |
| `src/triangle.rs` | 2, 3 |
| `src/vec3.rs` | 3, 5 |

Every changed path is surveyed. Requirements items 7 to 10 are each covered by at least one area.

## Confirmed clean facts

- One construction path: the only `HitRecord` literal is inside `HitRecord::new`
  (`src/hittable.rs:34-41`); `src/sphere.rs:44`, `src/plane.rs:44-51` and `src/triangle.rs:47` are the
  only callers.
- Face side and normal: `front_face = ray.direction.dot(outward_normal) < 0.0` and the normal is
  flipped from the outward normal when back-facing (`src/hittable.rs:28-33`); derives are
  `Clone, Copy, Debug` (`src/hittable.rs:9`).
- Texture coordinates match item 8: sphere latitude/longitude from the outward normal
  (`src/sphere.rs:39-43`, `u` in `(0,1]`, `v` in `[0,1]`), plane dominant-axis position with
  `rem_euclid(1.0)` (`src/plane.rs:35-50`), triangle barycentric weights after the range rejects
  (`src/triangle.rs:33-41,47`). No coordinate is taken from the ray-facing normal or recomputed.
- Plane and triangle match the B2 contract: `Plane::new` normalises (`src/plane.rs:19`), parallel
  epsilon `1e-9` (`:27`), strict `t_min < t < t_max` (`:31`); Möller-Trumbore with determinant epsilon
  `1e-8` and barycentric bounds (`src/triangle.rs:27-41`); both two-sided, no material, `Rc` or second
  hit path.
- Bounding boxes match the B3 contract: `Aabb::new` orders per axis (`src/aabb.rs:14-19`), the slab
  test rejects a missed box over `t_min..t_max` and handles an exactly parallel ray
  (`src/aabb.rs:22-47`); `bounding_box` is a required trait method (`src/hittable.rs:49-50`); sphere
  and triangle report boxes, plane and list report none (`src/sphere.rs:48-51`,
  `src/triangle.rs:51-62`, `src/plane.rs:55-57`, `src/hittable.rs:84-86`); the list tests the box over
  the full range before the object with the running limit, so the chosen record cannot change
  (`src/hittable.rs:69-78`).
- Scene and output match B2: sphere `(0,0,-1)` r 0.5, plane `(0,-0.5,0)` normal `(0,1,0)`, triangle
  `(-2,-0.5,-2)`, `(2,-0.5,-2)`, `(0,1.5,-2)` (`src/main.rs:37-50`); rows render top to bottom at
  200 by 100 (`src/main.rs:25-26,57-61`); one `write_ppm` on locked stdout (`src/main.rs:71-93`); no
  material, `Rc`, command line, PNG or progress output on this branch.
- Tests and docs: 13 test functions, added 2 by B1r, 3 by B2 and 3 by B3, each within the cap
  (`src/aabb.rs:54`, `src/hittable.rs:95`, `src/main.rs:99,106`, `src/plane.rs:64`, `src/rng.rs:29`,
  `src/sphere.rs:58,70,82,93`, `src/triangle.rs:70,88,99`); requirements items 7 to 10 and
  `README.md` are accurate for this branch; the repository-map omission is recorded in
  `docs-pass.md`.

## Findings

1. **Four `#[allow(dead_code)]` attributes have no product consumer on this branch**
   (`src/hittable.rs:13,16,18,20`). `point` is never read (its comment explains it as the bounce-ray
   origin); `front_face`, `u` and `v` are read only by `#[cfg(test)]` assertions
   (`src/sphere.rs:67,78,89-90`, `src/plane.rs:70,72-73`, `src/triangle.rs:82,84-85`); `main.rs`
   reads only `hit.normal` (`src/main.rs:31`). Expected to become live in the release merge, but on
   this branch the suppressions are forward-looking.
2. **Sphere texture `v` can become NaN at a pole** (`src/sphere.rs:40`): `acos` is unclamped on a
   normal that is only floating-point unit length. Not observed; the contract forbids clamping.
3. **`Plane` and `Triangle` expose `pub` fields** (`src/plane.rs:9-12`, `src/triangle.rs:9-13`), so a
   struct literal can bypass `Plane::new`'s normalisation. Contract-conformant latent interface risk;
   in-tree only `new` is used.
4. **Box tests cover fewer cases than the B3 prose describes**: no test for corner ordering, for
   `t_min..t_max` rejection, for the sphere's box, or a direct `None` assertion for `Plane` and the
   list (`src/aabb.rs:55-61`, `src/hittable.rs:96-108`). The suite count is within budget.
5. **Float boundary in the box test**: inclusive slab endpoints with no epsilon could in principle
   reject a true triangle hit that rounds marginally outside the vertices' f64 hull. Not observed,
   untested.
6. **`docs/requirements.md:19` names the module `src/rng.rs` inside a product requirements page.**
   Unchanged from the base commit.
7. **Plan-c's release list is stale for two items this branch already resolved**:
   `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md:26-34` still assigns the `main.rs` rng comment
   (resolved by `281dda8`) and the `hittable.rs` point comment (resolved by `faac698`) to plan-a's
   release. Plan-b's own `docs-pass.md` correctly lists only the repository map. Cross-plan record
   bookkeeping for the release, not a product defect.
8. **Repository map** in `AGENT-INSTRUCTIONS/LOCAL/project.md:5` omits `aabb.rs`, `plane.rs` and
   `triangle.rs` (and names no `material.rs`); already recorded as the release's single item in
   `AGENT-INSTRUCTIONS/PLANS/plan-b/docs-pass.md:7-12`.

## Out-of-scope observations

- `src/main.rs:95-110` tests call the private `render` helper and render the full 200x100 image while
  asserting two pixels; both predate this plan.
- All test modules sit inline in `#[cfg(test)] mod tests`; this is the repository's existing
  convention.
- `src/main.rs:30` uses an inline `0.001` hit epsilon rather than a named constant; acceptable for
  this project.

## Cross-check outcome

The cross-check confirmed almost every claim and returned these corrections and additions.

Rejected claims:

- Area 1, claim 17: the evidence bullet "no `point` field access at all" is false; `src/plane.rs:30`
  reads `Plane.point`. Correct: no `HitRecord.point` read exists.
- Area 4, claim 13: "the byte-identical-image claim is not evidenced" is false.
  `/tmp/opencode/b3scratch.ppm` is byte-identical to `/tmp/opencode/b2.ppm`, `b2rev.ppm` and
  `z1-accept.ppm` (all md5 `4e0a5f55aeec3f42d871df3255478508`, 238545 bytes; `b2.ppm` at 03:12 for
  `9afa2db`, `b3scratch.ppm` at 03:34, `z1-accept.ppm` at 03:46 for `c25007c`). Residual gap: no
  in-tree label ties those files to the two commits.

Corrected claims:

- Area 1, claim 9: the `point` allow is explained by its comment
  (`src/hittable.rs:12`), but has no consumer.
- Area 2, claim 12 and Area 3, claim 13: `rem_euclid(1.0)` is at `src/plane.rs:49-50`, not `50-51`.
- Area 3, claim 12: the B2 contract's future-looking lines include the release merge's material
  arguments (`B2-planes-and-triangles.md:28-31`), which have not landed on this branch.
- Area 3, claim 22: the triangle box test is the B3 test, and the contract reference is
  `B3-bounding-boxes.md:37-40`, not `:24`.
- Area 4, claim 28: the glob `b3-*.ppm` matches nothing, but `/tmp/opencode/b3scratch.ppm` exists and
  is the relevant artifact (Area 4 claim 29 compared it with the wrong base-scene render).
- Area 5, claim 6: in the success path no other output exists, but `fn main() -> io::Result<()>` lets
  the runtime print `Error: <debug>` to stderr when a write fails.
- Area 5, claim 15: the `main.rs` diff also adds `mod aabb;`, `mod plane;`, `mod triangle;` and their
  `use` lines.
- Area 6, claim 6: both `main.rs` tests call the private `render` helper (`src/main.rs:101,108`), so
  "no private-helper assertions" is false; the return itself notes this later.
- Area 6, claim 19: the B2 tests were added at `src/plane.rs:58` and `src/triangle.rs:54,72` in
  `9afa2db`; the numbers `71,89` are head function lines.
- Area 6, claim 25: the base item 6 text is at `docs/requirements.md:19`, not line 6.

Added claims the areas missed: the B3 byte-identical evidence exists as `/tmp` files (above); Area 4
searched the wrong glob and comparison base; Area 1's `.point` evidence overreach; the `main()` stderr
path; the `main.rs` module/import diff content.

Unconfirmed statements (no run was made, per the survey's read-only rule): build/clippy/test status,
the sphere-pole NaN boundary, the box float boundary, the rendered image's visual correctness from a
scout's view, and whether stale plan-c release entries are plan-b's to clear.

## Acceptance render

Run from the worktree root by the orchestrator:

```bash
~/.cargo/bin/cargo build --release
./target/release/toy-raytracer > /tmp/opencode/z1-accept.ppm
./target/release/toy-raytracer > /tmp/opencode/z1-accept-2.ppm
cmp /tmp/opencode/z1-accept.ppm /tmp/opencode/z1-accept-2.ppm
head -2 /tmp/opencode/z1-accept.ppm
```

- Release build finished; both runs exited 0; `cmp` printed nothing (238545 bytes each, md5
  `4e0a5f55aeec3f42d871df3255478508`).
- Header: `P3`, then `200 100`.
- Viewed after converting to `/tmp/opencode/z1-accept.png` and opening it with the environment's
  image reader: sky gradient across the top, the normal-shaded centre sphere, one flat light-green
  ground across the lower image (constant ground normal), and the light-blue triangle standing behind
  the sphere with both slopes beside it and its apex above it. This matches requirements items 7 to
  10 for this branch.

## For the user

- The survey found no product defect: the branch matches its three frozen contracts and the shared
  designs, and every changed file and requirement item 7 to 10 is covered.
- Decide whether to accept the survey and the rendered scene. Nothing below was fixed.
- Findings to note when deciding: the four forward-looking `dead_code` suppressions (finding 1), the
  sphere-pole NaN boundary (2), the public shape fields (3), thinner box-test coverage than the B3
  prose implies (4), the box float boundary (5), the module name in the requirements page (6), the
  stale plan-c release entries (7), and the repository-map release item (8).
- A finding accepted as a defect becomes a remediation round after this acceptance; it is not fixed
  in Z1.

## User decision

**Accepted 2026-11-24.** The user read the note and opened the rendered image, and accepted the
survey and the scene. No remediation round follows: findings 1 to 6 are accepted as documented (the
forward-looking `dead_code` allowances are expected to become live at the release merge, and the
pole and floating-point boundaries are unobserved and outside this plan's scope); findings 7 and 8
are release bookkeeping that the merging release reconciles. Accepted gaps are exactly those
findings.
