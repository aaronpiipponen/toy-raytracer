# Closing survey - plan-a (materials)

- **Surveyed revision:** branch `plan-a`, range `e180476..566b11c`, head clean at `566b11c`.
- **Areas:** materials (`src/material.rs`); hit record and sphere (`src/hittable.rs`,
  `src/sphere.rs`); render loop, generator and scene (`src/main.rs`, `src/rng.rs`); tests and
  documents (the 12 tests, `docs/requirements.md` items 1 to 6, `README.md`, `docs-pass.md`).
- **Method:** four independent read-only area scouts in parallel, then one independent read-only
  cross-check of every claim. Returns saved unchanged in `benchmarks/returns/`: `area-1-materials.md`,
  `area-2-hit-record-and-sphere.md`, `area-3-render-loop-generator-and-scene.md`,
  `area-4-tests-and-documents.md`, `cross-check.md`.
- **Outcome in one line:** no broken behaviour, no data loss, no hack and no plan reference in a
  permanent product file. The requirements 1 to 6 behaviour is present and correct. What remains is
  test-coverage gaps, a few small cleanups, and two frozen plan records that went stale.

## Confirmed findings

### Test coverage (REPO-RULES §3a)

1. **Diffuse randomness unproven** - `src/material.rs:147-154`. The diffuse test draws three times but
   only asserts `direction.dot(normal) > 0`. A constant direction would pass, so the "random direction
   around the normal" half of requirement 2 is not actually proven. *Class: test gap.*
2. **Metal fuzz blur and the glass Schlick branch are untested** - `src/material.rs:164` tests only
   `fuzz = 0.0` (exact mirror), `:185` only `fuzz = 1.0` (absorption); the Schlick-reflect arm at
   `src/material.rs:82` is never taken (the TIR test short-circuits on the critical angle, the Snell
   test's first draw `0.8202` exceeds `schlick ≈ 0.0403`). *Class: test gap.* A1 and A2 each already
   spent the three-test cap, so closing these needs a user decision, not an implementer.
3. **Whole-image render assertions** - `src/main.rs:141` (`assert_eq!`) and `:146` (`assert_ne!`)
   compare the whole 20×10 pixel vector, against the domain exception "rendering tests check a few
   pixels of a small image, never a whole image". The determinism test needs only a few pixels; the
   sample-count test proves the `samples` argument is read, not that pixels are averaged. *Class: test
   shape.*

### Cleanup

4. **`Rng::new` silently rewrites seed 0 to 1** - `src/rng.rs:9` `seed.max(1)` maps `Rng::new(0)` onto
   `Rng::new(1)` instead of failing. No product caller passes 0 (`SEED = 7` is the only call), so there
   is zero behavioural impact, but it is a silent substitution the repo rules dislike. *Class:
   unnecessary defensive substitution.*
5. **Record fields are public and mutable** - `src/hittable.rs:11-21`. Any consumer can assign `normal`
   or `front_face` after construction, so "the constructor is the only place a normal is flipped" is
   convention-only. Nothing in `src/` does this today (`grep '\.normal *='` finds no assignment). *Class:
   latent drift, no live violation.*
6. **Two ways to fill the scene list** - `src/hittable.rs:60` public `objects` and `:64-66` `add()`.
   Only `add()` is used anywhere. *Class: unnecessary surface.*
7. **Repeated sphere test setup** - `src/sphere.rs:73-77,91-95,109-113` build the identical
   `Sphere::new(…, Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))))`; a small local helper would
   remove the repetition. *Class: test duplication.*

### Stale frozen records (plan folder, not product docs)

8. **`done/A3r-samples-resolution.md:8` and `:42-43`** still say A3 "sits at `planned`" / "A3 remains
   `planned`". A3 is `done`, its contract is `done/A3-samples-per-pixel.md`, and `phases/` holds only
   the closing survey. *Class: stale record.*
9. **`done/A1-diffuse-and-metal-materials.md:31-32`** still says the record "carries no face-side flag"
   and describes the "outward normal". A2 replaced both. *Class: stale record.*

### Naming split in the merge designs

10. **`colour` vs `pixel`** - `PLANS/shared/plan-a-A3+plan-c-Z1.md:22,28` writes the merged pixel as
    `let mut colour` / `colour / samples as f64`, while `done/A3-samples-per-pixel.md:11` and
    `src/main.rs:89,95` say `pixel`. The release must pick one; otherwise one document and the merged
    code disagree on spelling. *Class: drift (naming only).*

## Rejected claims

- **`src/material.rs:121` `1e-12` epsilon "unnecessary"** - rejected as stated. The probability of a
  rejected sample below it is ~`5.2e-10`, not `1e-12`, and the guard is not removable: the generator
  can return exactly `0.5`, so the summed vector can be exactly zero and `.unit()` would be `NaN`. It
  is an undocumented defensive constant, not a dead branch.
- **"`#[allow(dead_code)]` remains only on `HitRecord.u`/`.v`"** - false as written; `src/vec3.rs:22`
  also carries one (base code, not this plan).
- **"A2 explicitly flagged A1's face-side lines as carry-over"** - false; A2 never mentions A1. The
  supersession at `done/A1:31-32` is real but implicit, which is why finding 9 above stands.
- **"`docs/requirements.md` items 1 to 6 all match the branch"** - true except item 5's "configurable
  number of samples", which is the merged product's; the branch has a fixed `SAMPLES = 16`.
- **`LOCAL/project.md` map is "stale"** - not for the merged product: it is the union of this branch's
  and the other plans' trees, and `docs-pass.md:15-19` already owns re-checking its `material.rs` line
  after the merge.
- **Seed `7` hardcoded in unit fixtures** - a fixture pinning its own seed is legitimate and cannot
  break anything; not a finding after review.

## Out-of-scope observations

- **`src/vec3.rs:21`** - `// For the triangle normals the geometry plan adds.` A permanent comment
  naming a plan. The integration tree already drops it; not a plan-a defect. Already in `docs-pass.md`.
- **`README.md:6`** - `cargo run > image.ppm` is correct for this branch, but after the merge the
  default output is a PNG file via `--output`, and the merge takes the output plan's README. A
  release-time reader should confirm the README; not listed in `docs-pass.md` today.
- **Requirement 12 (command line) and 13 (progress line)** - absent on this branch, by scope. Both are
  the output plan's, arriving through the two shared designs at the release merge.
- **`src/main.rs:1`** module header says "standard output as a PPM image" - accurate now, replaced at
  the merge by the output plan's header.

## User-review list

Nothing here needs a phase of its own. For the user's decision:

- **Test-coverage gaps (findings 1 to 3).** Closing the diffuse-randomness, fuzz-blur and Schlick
  coverage costs test functions beyond the §3a cap already spent in A1 and A2. Options: accept the gap
  as the hobby-project bar (the behaviour is present and the image proves it), or authorize a small
  remediation with the named extra tests.
- **Cleanups (findings 4 to 7, 10) and stale records (8, 9).** Small, one-line-each fixes: fail loudly
  on seed 0 or document the clamp, drop the unused public `objects` or keep it, add a sphere test
  helper, fix two stale frozen-record sentences, and pick `pixel` or `colour` in the merge design.
  Recommendation: fix these in a short remediation round; none changes behaviour.

## Rendered-image evidence

- Command: `~/.cargo/bin/cargo run --release > /tmp/opencode/plan-a-acceptance.ppm` at head `566b11c`,
  converted to `/tmp/opencode/plan-a-acceptance.png` and opened.
- The opened image (200 by 100) shows the glass sphere on the left refracting and inverting the sky and
  ground through it, the red-brown diffuse centre sphere with smooth shading, the grey diffuse ground,
  and the grey metal sphere on the right reflecting sky above and ground below. Sphere edges blend over
  several pixels, the sky and the flat sphere areas show no salt-and-pepper grain, and there are no
  flat `NaN` grey or pure-black pixels.
- This matches requirements 1 to 6 and the frozen A1, A2 and A3 contracts.
