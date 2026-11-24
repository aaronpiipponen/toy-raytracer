## Area 4 report — tests and documents, plan-a `e180476..566b11c`

### Findings

**1. stale document — `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A3r-samples-resolution.md:8` and `:42-43`**
`A3r` still says `phases/A3-samples-per-pixel.md` "sits at `planned`" and "Exit state: A3 remains `planned` … implemented after the pair returns `RESOLVED`". Both are now false: `full-plan.md`'s status row reads A3 `done`, the file is `done/A3-samples-per-pixel.md`, and `done/A3` is the landed contract at `566b11c`. A later reader gets a contradictory picture of the phase state from a "frozen" record.

**2. drift (doc-level only) — variable name in the shared render design vs the frozen contract and code**
`PLANS/shared/plan-a-A3+plan-c-Z1.md:22,28` shows the merged pixel as `let mut colour` / `colour / samples as f64`, while the frozen A3 contract (`done/A3-samples-per-pixel.md:11`) and the code (`src/main.rs:89,95`) use `pixel`. The release has two "authoritative" spellings for the same line, so the merge will silently have to pick one and one of the two documents will not match the merged code.

**3. test focus (§3a "one test function = one behaviour"/proportional proof) — weak or whole-image render tests**
- `src/main.rs:141` and `src/main.rs:146` compare the entire 20×10 pixel vector (`assert_eq!` / `assert_ne!`), contrary to the domain exception "Rendering tests check a few pixels of a small image, never a whole image". The determinism test only needs a few pixels; the sample-count test passes if a single pixel differs, so it proves the `samples` argument is read but not that pixels are *averaged* over `samples` rays.
- Both are the frozen A3 contract's own tests (`done/A3:22-25`), so this is a design tension to accept or remediate, not drift.

**4. test focus — a test named for randomness that cannot detect non-randomness**
`src/material.rs:147-153`: `a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface` draws three times but only asserts the *sign* of `dot(direction, normal)`. A constant direction would pass, so the "random direction" half of requirement 2 is not actually proven. Requirement 2 is user-visible behaviour the domain exception asks to cover.

**5. test focus — (u, v) assertions packed into the face-side tests**
`src/sphere.rs:84-86` and `:101-104` pack t, `front_face`, normal and `u`/`v` into one function each, while `src/material.rs:209-210` and `:231-232` re-assert `front_face`/normal flipping inside the material tests — behaviour already proven at the hit layer. Requirement 7 (face side) and requirement 8 (texture coordinates) are separate behaviours; the dedicated `(u, v)` test was deliberately dropped by `PLANS/shared/plan-a-A3+plan-b-Z1.md:100-101`, so the packing is the agreed compromise rather than drift. Flag for the user, low severity.

**6. coverage gap (user-review, phase cap blocks adding tests unasked)**
Requirement 3's "blurred by a fuzz factor" and requirement 4's Schlick branch are unproven: `src/material.rs:157-176` tests only `fuzz = 0.0` (exact mirror) and `:179-199` only `fuzz = 1.0` absorption; no test shows a non-zero fuzz turning the mirror direction into a different direction, and no test exercises the Schlick-reflect branch (`src/material.rs:82`) rather than the deterministic TIR/refract paths. A1/A2 each already used the 3-test cap, so this is a user decision, not an implementer's.

### No findings in these classes

- **unnecessary complexity** — none new in the changed code; `ray_colour`, `sky_colour`, `to_byte` and `render` are each one path.
- **dead code** — none: `#[allow(dead_code)]` remains only on `HitRecord.u`/`.v` (`src/hittable.rs:17-20`), mandated by `done/A2:12`; `front_face`, `point`, `t` and `material` are all read.
- **redundant check or record** — none; `render` carries no sample validation and `parse_args` is the single validation path, matching `done/A3:16-17`. The `record_with` test helper (`src/material.rs:133`) is a fixture, not a mock, and has no dedicated test.
- **fake product behaviour / packed unrelated behaviours** — none; every test calls the real material, sphere and render code.
- **docs-pass items made obsolete by this branch** — none. Item 1's fallback sentence is still *true* here (no command line); items 2-4 describe the integration tree; item 3's material-module line is accurate on this branch. The release-merge carry-over (triangle `Lambertian` `(0.7,0.3,0.3)`) is still needed and still correct: the branch has no triangle, and the albedo equals the front sphere's (`src/main.rs:60`).
- **README / requirements (items 1-6)** — accurate for this branch. `docs/requirements.md:8-19` matches `main.rs:26,33-46,85-95,101-103` (50 bounces → black, Lambertian/Metal/Dielectric rules, 16-sample default, gamma 2, fixed `SEED`). Items 11-13 describe the merged product and are plan-c's, as `done/A3r:23-26` records. `README.md:6`'s `cargo run > image.ppm` is correct here (ASCII PPM to stdout).
- **plan/phase references in code and doc comments** — only the known `src/vec3.rs:21` (below). The A1-era comments in `main.rs` and `hittable.rs` ("the plans add") are gone at `566b11c`.

**Non-finding, contract-sanctioned:** `src/main.rs:80-81` promises "the share of finished rows reported on standard error" while the branch writes no progress line. `done/A3:18-21` and `PLANS/shared/plan-a-A3+plan-c-Z1.md:60-61` state that this doc sentence intentionally describes the merged product and is edited only here. Do not reopen.

### Out-of-scope observations

1. `src/vec3.rs:21` — `// For the triangle normals the geometry plan adds.` A permanent comment naming a plan; the integration tree already drops it (per brief). Not a plan-a defect.
2. `README.md:6` — after the merge the default output is `render.png` via `--output` (`docs/requirements.md:37-38`), so this branch's `cargo run > image.ppm` example will no longer describe the released default. README is plan-c's contribution (`done/A3r:25-26`) and `docs-pass.md` does not list it; a release-time reader should check it.
3. `src/rng.rs:29-36` (`the_same_seed_gives_the_same_sequence`) overlaps the render-level determinism test `src/main.rs:140-142` ("a case already proven at another layer"), but it is a product module test of requirement 6 and predates this range (`git diff e180476..566b11c -- src/rng.rs` is empty). Not actionable here.
4. `src/main.rs:1`'s `//! … write it to standard output as a PPM image.` is accurate on this branch only; the merge replaces it with plan-c's header (`PLANS/shared/plan-a-A3+plan-c-Z1.md:84-85`). No action.
