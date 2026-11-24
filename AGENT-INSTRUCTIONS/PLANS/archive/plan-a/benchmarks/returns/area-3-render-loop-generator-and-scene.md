## Findings

**Duplication (minor)**
- `src/material.rs:146,166,187,211,233` and `src/rng.rs:31-32` — the seed `7` is written as a bare literal in five material tests and the rng test, duplicating `SEED` (`src/main.rs:25`) instead of referencing `crate::SEED`; a future seed change silently leaves the unit fixtures on the old value. Cosmetic in test code only.

**Unnecessary complexity / defensive substitution (minor)**
- `src/rng.rs:9` — `seed.max(1)` silently rewrites a caller's seed `0` to `1` rather than rejecting it, so `Rng::new(0)` and `Rng::new(1)` collapse onto the same stream. Reproducibility for the fixed `SEED = 7` is unaffected, but it is a silent fallback (against the "fail loudly, no silent substitution" rule) where an explicit assertion or documented pre-condition would be honest. Severity low; no product caller passes 0.

**Stale document**
- None in this area. `src/main.rs:80-81`'s doc sentence claims the share of finished rows is reported on stderr while branch `plan-a` writes no progress line; this is explicitly frozen and sanctioned by the A3 contract (`A3-samples-per-pixel.md:18-21`) to describe the merged product, so it is intentional, not a defect.

**No findings in these classes**
- *Dead code:* the only `#[allow(dead_code)]` under `src/main.rs`/`src/rng.rs` history was removed by the range (`mod rng` lost its allow in `e180476..566b11c`); nothing dead remains in either file.
- *Redundant check or record:* no second re-check of the bounce result or the hit; `ray_colour` (`src/main.rs:30-47`) has exactly one `scatter`/`None` branch and one loop exit.
- *Second render path:* only one `render` (`src/main.rs:82`) and one `write_ppm`; `grep 'fn render\|render('` shows only its definition, `main`, and the three tests.
- *Test bloat:* three tests in `main.rs` (12 total in the crate), each mapping to requirement 1/5/6; runtime 0.35 s. No overlap.

**Contract conformance (checked, all pass)**
- Bounce loop runs `MAX_BOUNCES = 50` scatters then returns black (`src/main.rs:26,33,46`); absorption returns black at `:40` — matches requirement 1.
- `sky_colour` defined `:50`, used exactly once at `:43`.
- `to_byte` clamps then `sqrt()` (gamma 2) at `:102`, applied on all three channels `:116-118` — requirement 5.
- One `Rng::new(SEED)` at `:85`, before both loops, never reseeded; `u` drawn before `v` (`:91-92`) then the sample's scattering draws; pixel pushed as `pixel / samples as f64` (`:95`) — contract lines 6-11 exactly.
- `SAMPLES = 16` directly below `HEIGHT` (`:24`), used only by `main` (`:125`); no test names `WIDTH/HEIGHT/SAMPLES`; `render` carries no sample-count validation.
- Four spheres with Lambertian×2, Metal, Dielectric (`:57-76`) — requirement 1/2/3/4 scene.
- Seed `7` fixed at `:25`; no time/OS/env source (`grep` for `SystemTime|Instant|getrandom|/dev/urandom|std::env|thread_rng|rand::` and `Cargo.lock` dependency count = 1) — requirement 6. All randomness flows through `Rng` into `material.rs:82,116-118`.
- Byte-for-byte reproducibility holds: single sequential deterministic stream, stateless `Rc<dyn Material>` (all `scatter` take `&self`), and `two_renders_with_the_same_options_give_the_same_pixels` asserts it.
- Test budget claim in the contract ("2 added, 0 deleted, 1 adapted") verified against `a1d640a..566b11c`.

## Out-of-scope observations (not investigated further)
- `src/hittable.rs:17-20` — `u`/`v` fields are `#[allow(dead_code)]`, unused on this branch; attributed to the geometry plan in the A3 carry-over.
- `src/vec3.rs:22-30` — `cross` is `#[allow(dead_code)]`, unused until the triangle plan.
- Requirement 13 (progress line on stderr) is absent from this branch; `src/main.rs:124-127` renders and writes PPM only. Documented as a release-merge carry-over (per contract lines 18-21 and 36-39), so not a branch defect.
- Requirement 12's command line and PNG output do not exist on this branch; consistent with scope (merge-time work).
