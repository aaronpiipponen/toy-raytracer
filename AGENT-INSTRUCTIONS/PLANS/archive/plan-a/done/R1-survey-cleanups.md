## Phase R1 - Survey cleanups [DONE]

Implemented and verified. Build instructions retired. Later work consumes these
frozen contracts:

- `Rng::new(seed: u64) -> Self` in `src/rng.rs` asserts a nonzero seed
  (`assert!(seed != 0, "the generator seed must be nonzero");`) and stores the seed unchanged. Zero
  is an error, never silently rewritten; every nonzero seed keeps the previous state sequence, so
  `SEED = 7` renders identical bytes. The doc comment states the panic.
- `HittableList.objects` in `src/hittable.rs` is private. `add(&mut self, object: Box<dyn Hittable>)`
  stays the one way to fill the list, and `HittableList::hit` reads the field inside the module. No
  accessor, constructor, clear method or second fill path exists.
- The three sphere tests in `src/sphere.rs` take their fixture from one local `fn grey_sphere() ->
  Sphere` (centre `(0.0, 0.0, -2.0)`, radius `0.5`, grey `Lambertian`); every assertion, ray and
  test name is unchanged.
- `done/A3r-samples-resolution.md` states A3's contract is `done/A3-samples-per-pixel.md` and A3 is
  `done`; `done/A1-diffuse-and-metal-materials.md` states the record's normal faces the incoming ray,
  carries `front_face`, and its shared shape is
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A2+plan-b-B2.md`. The rest of both frozen records is
  unchanged.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` names the per-pixel accumulator `pixel`
  in the merged `render` snippet, matching `done/A3-samples-per-pixel.md` and `src/main.rs`. No
  plan-c file changed.
- Verification at branch head `b22d647`, base `566b11c`: `cargo build` clean, `cargo test` 12
  passed 0 failed, `cargo clippy -- -D warnings` clean, `cargo fmt --check` clean, `git diff --check`
  clean. The rendered-image conditional gate ran once at the default 200 by 100, 16 samples: the PPM
  is byte-identical to the pre-change render, both sha256
  `5457ef7a788539f5f5b0145e7c20fd178a77f0d184663b83fcd38d299a54665b`. Test budget: 0 functions
  added, 0 deleted, 3 adapted.
- Unchanged by this phase: rendering and sampling behaviour, `SEED`, `MAX_BOUNCES`, the scene,
  `to_byte`, `write_ppm`, the materials, the `1e-12` guard, the public record fields, the `pixel`
  local name in `src/main.rs`, `done/A3-samples-per-pixel.md`, `docs/requirements.md`, `README.md`,
  `AGENT-INSTRUCTIONS/LOCAL/`, `Cargo.toml` (still no dependency), and no `unsafe` or threads.
- Accepted test-coverage gaps stay accepted: the diffuse-randomness, metal-fuzz and Schlick coverage
  reported by the closing survey is not closed, by the user's decision.
