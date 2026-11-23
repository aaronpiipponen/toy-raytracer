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
- The plan-a orchestrator owns A1r on plan-a's branch. **No cross-branch transfer exists**: commits
  never move between plan branches and no plan branch is merged, rebased or cherry-picked onto
  another; the releases merge both into `main` and resolve textual conflicts by the shared design.
  plan-a's A1r writes its own `HitRecord::new(t, point, normal, material)` with no `front_face`/`u`/`v`,
  plus the same `as_ref()` line in `HittableList::hit`.
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
