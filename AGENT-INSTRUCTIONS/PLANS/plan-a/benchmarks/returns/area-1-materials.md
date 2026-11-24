## Answer

`src/material.rs` satisfies requirements 1–4 and both frozen contracts: the `Material` trait is exactly the contracted `scatter(&ray_in, &hit, &mut Rng) -> Option<(Colour, Ray)>`; `Lambertian` scatters around the ray-facing normal by its albedo; `Metal` reflects the unit direction, adds `fuzz·random_unit_vector`, absorbs when `scattered·normal <= 0`, and returns its albedo; `Dielectric` picks the entering/leaving `eta_ratio` from `front_face`, reflects on `eta_ratio·sin_theta > 1.0` or one `rng.next_f64()` draw under `schlick`, refracts otherwise, and returns unit attenuation without ever returning `None`. All four private helpers are single-purpose and used. I found no dead code, no test bloat, no stale material docs and no mid-file structural drift; the only in-area findings are one mild numeric duplication (frozen by the A2 contract) and one unnecessary epsilon guard.

## Evidence

- Duplication (accepted/frozen): `-v.dot(n)` with `.min(1.0)` is computed in `scatter` (`src/material.rs:79`) and again inside `refract` (`src/material.rs:99`), and the same grazing sine appears twice (`src/material.rs:80` vs the `1 - perpendicular·perpendicular` inside `refract`). The A2 contract freezes `refract(v, n, eta_ratio)` as self-contained, so this is duplication by design, not a defect.
- Redundant check / magic constant: `src/material.rs:121` `if len_squared > 1e-12 && len_squared <= 1.0` — the `1e-12` epsilon guards a near-zero sample that rejection sampling can essentially never produce from 53-bit draws (cube-volume probability ~1e-12); `<= 1.0` is the only needed bound. Harmless but an undocumented defensive constant.
- Contract conformance: `src/material.rs:11` matches A1's trait; `:28-29` Lambertian; `:48-53` metal reflect/fuzz/absorb/albedo; `:74-87` `eta_ratio` from `front_face` + ray-facing normal, TIR `:82`, Schlick `:82`, refract `:85`, unit attenuation `:87`; `:92-125` the four private helpers, each used exactly for its one job.
- No dead code: every helper (`reflect` `:92`, `refract` `:98`, `schlick` `:106`, `random_unit_vector` `:113`) is called; the only unused parameter is the trait-mandated `_ray_in` (`:27`).
- No test bloat: five tests, each asserting a requirement-level behaviour (diffuse, mirror, fuzz absorption, TIR, Snell); the fuzzy-absorption loop is deterministic under seed 7.
- Confirmed-correct math: `schlick`'s `r0 = ((1-eta)/(1+eta))²` is ratio-symmetric, so it is right for both entering and leaving; TIR test `eta_ratio*sin_theta > 1.0` is Snell's critical angle (`src/material.rs:82`).

## Out-of-scope observations

- Stale document / drift: `AGENT-INSTRUCTIONS/LOCAL/project.md` "Repository map" lists `src/aabb.rs`, `src/plane.rs`, `src/triangle.rs`, but `ls src/` shows only `camera, hittable, main, material, ray, rng, sphere, vec3` — the map describes a future state (requirements 9–10) not present at 566b11c.
- Drift vs requirements: `src/main.rs:1` says it writes "to standard output as a PPM image", and `main.rs:124-127` has no argument parsing; requirements 11–13 require PNG, `--width/--height/--samples/--output/--format`, default `render.png`, and a stderr progress line. `main.rs:22-23` defaults are 200×100 vs requirement 12's 400×200. Likely later areas, flagged only as existing evidence.
- `src/hittable.rs:17-20` `#[allow(dead_code)]` on `u`/`v` is intentional per the A2 contract (no texture consumer yet), not a defect.

## Unknowns

- I did not run `cargo build/test/clippy/fmt` (read-only), so requirement 14 is unverified by this survey.
- Theoretical only: `Lambertian` (`:28`) can in principle return a zero-length direction if `random_unit_vector` landed exactly on `-normal`; the probability is measure-zero for real inputs, so it is not a realistic defect under the hobby-project rule, just noted.
