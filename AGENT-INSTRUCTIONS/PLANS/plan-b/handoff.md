# Orchestrator handoff

## Next action

When plan-a sends its detailed `A1r` phase file, run the orchestrator acceptance check on it against
`PLANS/shared/plan-a-A1+plan-b-B2.md`, then call `channel_close` naming it. Then spawn
`phase-implementer` for B2r and after that re-check B2 before its implementer.

## Current state

- B1: `done` at `6eb43e09`.
- B2r: `active`, detailed at `phases/B2r-shared-hit-record-resolution.md`, accepted by the
  orchestrator: one line, `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`, gate minimal,
  no test function, no conditional gate (output cannot change). Not implemented.
- B2: `planned`, re-detailed at `phases/B2-planes-and-triangles.md`, accepted: adds `src/plane.rs`,
  `src/triangle.rs`, one triangle to the scene, three new tests, deletes the two plan-naming comments
  and the `Vec3::cross` dead-code marker, updates the project.md repository map.
- B3, Z1: `planned`.
- plan-a's `phases/A1r-hit-record-constructor-and-materials.md` currently holds only the phase
  template, so plan-a's detail is still in progress. Do not close the channel until it is real.
- Autonomous mode on; standing authorization for phase commits exists. No open user decision.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp`), never a remembered
  pixel value, when the claim is that the image did not change. A specific pixel probe is acceptable
  only when a diff cannot prove the new behaviour (for example, that a newly added object renders).
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
- Deleting comments that name a plan or a later stage (REPO-RULES §4.13) is legitimate work inside a
  phase that already touches the file; keep it to the comment lines only.
- A `Rc<dyn Trait>` field kills `Copy` and `Debug` on a derived struct; do not add `Debug` as a trait
  supertrait just to keep a derive when no reader prints the value.
- Commits never move between plan branches; parallel plans each implement their own side on their own
  branch and the release merges both into `main`, reconciling textual conflicts by the shared design.
  Never offer a cross-branch tip as a hand-off.
- A phase file that still contains `<angle-bracket placeholders>` is a template, not a detailed
  phase; do not accept it or close a channel on it.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp`), never a remembered
  pixel value, when the claim is that the image did not change. A specific pixel probe is acceptable
  only when a diff cannot prove the new behaviour (for example, that a newly added object renders).
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
- Deleting comments that name a plan or a later stage (REPO-RULES §4.13) is legitimate work inside a
  phase that already touches the file; keep it to the comment lines only.
- A `Rc<dyn Trait>` field kills `Copy` and `Debug` on a derived struct; do not add `Debug` as a trait
  supertrait just to keep a derive when no reader prints the value.
- Commits never move between plan branches; parallel plans each implement their own side on their own
  branch and the release merges both into `main`, reconciling textual conflicts by the shared design.
  Never offer a cross-branch tip as a hand-off.
