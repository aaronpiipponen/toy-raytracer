# Orchestrator handoff

## Next action

B2's implementer is running (`phase_subagent`, phase id B2) after the pair resolution. When it
returns, check that its review loop finished, then start `REVIEW AND ADVANCE` for B2 with a fresh
advancer.

## Current state

B1 done (`done/B1-face-side-and-texture-coordinates.md`). Pair `plan-a-A2+plan-b-B2` is RESOLVED;
the shared design is `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A2+plan-b-B2.md` and B2
(`phases/B2-planes-and-triangles.md`) is the accepted plan-b resolution phase file. B2 stays
material-less on this branch: `Plane`/`Triangle` with no material placeholder, records through the
six-argument `HitRecord::new`, `HittableList::hit` Copy-based, two test functions. The release merge
adds `material` to both shapes and their tests and gives the scene triangle a material; that
material choice is a user-owned decision recorded in the shared design (recommendation: the same
diffuse as the scene spheres) and must be asked at release, not picked.

Orchestrator acceptance check on B2 passed on 2026-11-24: scope matches requirement items 8 and 9;
the plane and triangle hit maths, the (u, v) wrap and both test expectations were verified by hand;
the scene triangle sits entirely in sky to the right of the sphere (base above the ground horizon,
apex below the top edge, columns ~130-196); test budget is 2 added (I merged two same-fixture
triangle tests into one); no records, receipts or defensive branches were added.

Open item: the shared repository map `AGENT-INSTRUCTIONS/LOCAL/project.md:5` already lists
`material.rs`, which does not exist in this worktree (plan-a owns it). B2 step 9 appends `plane.rs`
and `triangle.rs` and keeps existing entries; watch for a concurrent shared-file edit.

## Learned rules

- The user enabled autonomous mode for the routine loop: settle small, easily undoable,
  behavior-neutral choices myself and record each with its reason and how to undo it in
  `autonomous-decisions.md`. Ask the user only for product decisions, a user test or acceptance, and
  plan closure.
- Scope authorizes phase commits on branch `plan-b`: commit whenever useful, never squash, never to
  `main`.
- Test-cap hygiene is part of my acceptance check: merge near-duplicate test functions that share one
  fixture instead of spending the cap.
- Parallel-plan splits: each branch stays compiling on its own; obligations that only the release
  merge can satisfy live in the shared design file, never as placeholders in code.
