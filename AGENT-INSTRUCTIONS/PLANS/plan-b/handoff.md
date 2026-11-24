# Orchestrator handoff

## Next action

Make the B2 `phase_subagent` call now that B2 passed the acceptance check. The coordinator hold
`plan-a-A2+plan-b-B2` is armed, so that call is expected to be refused with `COORDINATOR HOLD`; the
refusal records this plan's stop and the channel opens only after both plans have stopped. On the
refusal: start nothing else, tell the user in one line, end the turn, and wait for the coordinator or
the partner orchestrator.

## Current state

B1 is done (`done/B1-face-side-and-texture-coordinates.md`), status row flipped. B2 is detailed at
`phases/B2-planes-and-triangles.md`, status `active`, audited twice by the advancer (second pass
PASS), and passed the orchestrator acceptance check on 2026-11-24 with one correction applied by the
orchestrator: the two triangle tests sharing one fixture and ray were merged into
`a_ray_through_the_triangle_hits_its_front_face_with_barycentric_coordinates`, so B2 now adds 2 test
functions (within the §3a cap) and the fast gate counts 9 tests.

B2 is held: plan-a leads the settlement of the shared `HitRecord` shape (plan-a's record carries
`Rc<dyn Material>` and is not `Copy`; ours carries `front_face`/`u`/`v` built only through
`HitRecord::new`). B2's detail must be re-detailed after the shared design lands; its materials-less
`HitRecord::new` assumption will not survive unchanged.

Open item surfaced by the advancer: the shared repository map
`AGENT-INSTRUCTIONS/LOCAL/project.md:5` already lists `material.rs`, which does not exist in this
worktree (plan-a owns it, and this line is shared at the workspace root). B2 step 9 keeps existing
entries and appends `plane.rs` and `triangle.rs`; the concurrent shared-file edit is a real risk to
watch, not a B2 defect.

## Learned rules

- The user enabled autonomous mode for the routine loop: settle small, easily undoable,
  behavior-neutral choices myself and record each with its reason and how to undo it in
  `autonomous-decisions.md`. Ask the user only for product decisions, a user test or acceptance, and
  plan closure.
- Scope authorizes phase commits on branch `plan-b`: commit whenever useful, never squash, never to
  `main`.
- Test-cap hygiene is part of my acceptance check: merge near-duplicate test functions that share one
  fixture instead of spending the cap.
