# Orchestrator handoff

## Next action

None. The plan is released and archived; its work is complete.

## Current state

- Every status row is `done` or `not applicable`: A1r, A2, A3 and Z1 done, A1 `not applicable`.
- Released on branch `plan-a` at `de4eb02`, one comment reflow after the tested head `dca633c`, and
  merged into `main` at `9afaa53` with plan-b and plan-c already merged there.
- The merge conflicted in `src/hittable.rs`, `src/main.rs` and `src/sphere.rs`, and every conflict
  resolved by `PLANS/shared/plan-a-A1+plan-b-B2.md`: one record with both sides' fields, `Clone`
  only, one constructor with the material last, `Dielectric` reading `hit.front_face` and the
  ray-facing normal, and `Plane` and `Triangle` carrying their own material. No conflict needed a
  choice the shared design does not make.
- Merged `main` passes `cargo build`, `cargo test` (23 passed), `cargo clippy -- -D warnings`,
  `cargo fmt --check` and `git diff --check`. The rendered-image gate opens and shows the sky, the
  ground, the four spheres and the triangle. Two renders with the same options are byte-identical.
- The release documentation pass reflowed the metal fuzz doc comment at `de4eb02` and corrected the
  two normal comments in the merged `src/hittable.rs`.
- The root `README.md` was rewritten for the whole program at `850f87f`.
- No `docs-pass.md` existed; the plan deferred no page work to the release.
- Survey note at `benchmarks/survey-2026-11-23.md` with 21 findings recorded and none scheduled, as
  the user directed. `AGENT-INSTRUCTIONS/LOCAL/project.md` still omits `src/material.rs` from its
  repository map; it is agent material and no fix is authorized.
- Scratch renders in `/tmp/opencode` removed.

## Learned rules

- A3 makes the sample count a parameter of `render` with 16 as the internal default; the command
  line exposes `--samples`.
- No cross-branch code transfer and no branch merges another; a shared design only aligns what each
  side writes on its own branch, and the release merges both into main.
- A resolution phase carries the held phase's work; the held row becomes `not applicable` rather
  than staying as an empty planned phase.
- A coordinator hold armed after an implementer has already started is not valid; resume that same
  implementer session rather than restarting the phase.
- Render artifacts left in `/tmp/opencode` are reused by a phase's user test, never written into the
  repository, and are removed once the test is accepted.
- The closing survey is mine to run: parallel area scouts, one cross-check scout, then I write the
  dated note; the plan closes only on the user's acceptance.
- Workspace rule: rendering tests check a few pixels of a small image, never a whole image
  (`LOCAL/repo-rules-domain.md:37`).
- Plan closure and archiving belong to the release; I never archive the plan myself.
- A merged hit record that carries a material and a ray-facing normal makes a material read
  `front_face` instead of testing the incident direction. Its tests build the record through the
  one constructor, so a ray-facing normal needs a ray and an outward normal, and an absorption case
  needs fuzz at grazing incidence.
