# Orchestrator handoff

## Next action

B3's implementer is running (`phase_subagent`, phase id B3). When it returns, check that its review
loop finished, then start `REVIEW AND ADVANCE` for B3 with a fresh advancer.

## Current state

B1 and B2 are done (`done/B1-face-side-and-texture-coordinates.md`,
`done/B2-planes-and-triangles.md`); B2 landed at `f3a3d2a` with the plane, the triangle, the scene
triangle and the repository-map update. B3
(`phases/B3-bounding-boxes.md`) is detailed, audited (two header self-contradictions fixed by the
advancer), and passed the orchestrator acceptance check on 2026-11-24: it matches requirement item
10; the slab predicate, the list-dispatch stub test and the triangle box numbers were verified by
hand; the byte-identical render claim is sound because every object's box contains it; 3 tests added
(the §3a cap) and no caching, receipts or defensive branches.

Open items:
- The release must ask the user which material the scene triangle gets (recorded in
  `PLANS/shared/plan-a-A2+plan-b-B2.md`).
- The `/workflow-reload` note for the shared `project.md` edit is unresolved; mention it to the
  user, never run it unasked.
- The shared repository map is edited by both plans; watch for a concurrent edit.

## Learned rules

- The user enabled autonomous mode for the routine loop: settle small, easily undoable,
  behavior-neutral choices myself and record each with its reason and how to undo it in
  `autonomous-decisions.md`. Ask the user only for product decisions, a user test or acceptance, and
  plan closure.
- Scope authorizes phase commits on branch `plan-b`: commit whenever useful, never squash, never to
  `main`.
- Test-cap hygiene is part of my acceptance check: merge near-duplicate test functions that share one
  fixture instead of spending the cap.
- Parallel-plan splits: each branch stays compiling on its own; obligations only the release merge can
  satisfy live in the shared design file, never as placeholders in code.
