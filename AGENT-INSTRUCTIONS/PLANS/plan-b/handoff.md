# Orchestrator handoff

## Next action

Spawn `phase-advancer` in DETAIL RESOLUTION mode:

`DETAIL RESOLUTION: Pair plan-a-A1+plan-b-B2. Plans plan-b (held phase B2) and plan-a (held phase A1). Shared design: PLANS/shared/plan-a-A1+plan-b-B2.md.`

Then run the orchestrator acceptance check on both resolution phase files (`plan-b` B2r and `plan-a`
A1r), call `channel_close` naming plan-a's resolution phase file, and continue with B2r.

## Current state

- B1: `done` at `6eb43e09`; `done/B1-face-side-and-texture-coordinates.md` amended so its derive list
  and `HittableList` line point at the shared design as superseded.
- B2: `planned` again (held before implementation, so no B2 tip exists); its phase document predates
  the resolution and must be re-detailed after B2r before any implementer.
- B2r: `active`, resolution phase, not yet detailed. Changes only `HittableList::hit` to
  `nearest.as_ref().map_or(t_max, |hit| hit.t)`.
- B3, Z1: `planned`.
- Shared design written: `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`. Agreed with the
  plan-a orchestrator: `Clone` only (no `Copy`, no `Debug`), material last in `HitRecord::new`, one
  constructor, `as_ref` in the list, plan-a owns the material field/parameter/call sites.
- The plan-a orchestrator agreed to take B1 across now, not B2, and to add a small later unit for
  materials on planes and triangles. **Coordinator approval for the B1 cross-branch hand-off is
  still required** (tip `6eb43e09`); I cannot authorise it myself. The user has been told.
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
- Never offer an uncommitted tip as a cross-branch hand-off; state exactly which commit is available.
