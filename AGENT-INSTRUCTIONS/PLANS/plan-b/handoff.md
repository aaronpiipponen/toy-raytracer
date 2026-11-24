# Orchestrator handoff

## Next action

B3's `REVIEW AND ADVANCE` advancer is running. It should detail Z1, the closing survey and user
acceptance phase. When it returns, read Z1's handoff and run my acceptance check on the Z1 detail.

## Current state

B1, B2 and B3 are implemented on branch `plan-b`; B3 landed at tested head `f85a183` (base
`f3a3d2a`). Receipt: build, 12 tests, clippy `-D warnings`, fmt, `git diff --check` green;
before/after renders byte-identical (md5 `13b3bc0a3cf5c41df684e179c1d3e93d`); 3 tests added;
review pass clean. B3's `done/` compression and status flip belong to the advancer.

After B3 the only remaining phase is Z1: the binding closing survey (one read-only `ops-scout` per
area in parallel, then one independent cross-checking scout, then the orchestrator writes the dated
note in `benchmarks/`), plus rendering the scene and asking the user to accept. Plan closure and the
release are not this session's work.

Open items to carry:
- Release must ask the user which material the scene triangle gets
  (`PLANS/shared/plan-a-A2+plan-b-B2.md`).
- `/workflow-reload` for the shared `project.md` edit is still a question for the user.

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
