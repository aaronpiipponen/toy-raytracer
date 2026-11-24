# Orchestrator handoff

## Next action

Z1's closing survey is starting: four read-only `ops-scout` area scouts run in parallel. Save each
return unchanged under `benchmarks/returns/`, then dispatch one cross-check scout, write the dated
note, render and inspect the acceptance image, and ask the user to accept.

## Current state

B1, B2 and B3 are done and green; B3 landed at tested head `f85a183` (base `f3a3d2a`). Z1
(`phases/Z1-closing-survey-and-user-acceptance.md`) is detailed, audited, and passed my acceptance
check on 2026-11-24: it matches the binding survey method (four areas in parallel, one cross-check,
orchestrator writes the note from saved returns), is read-only, fixes nothing, changes no product
code, adds no test, and carries a user acceptance gate for the note and the render. Saved returns are
mandated by my own orchestration rules, not an extra record.

Open items to carry:
- Release must ask the user which material the scene triangle gets
  (`PLANS/shared/plan-a-A2+plan-b-B2.md`).
- `/workflow-reload` for the shared `project.md` edits is still a question for the user.
- The shared repository map lists `material.rs`, which this branch does not have (plan-a owns it);
  area 4 should report it as an out-of-scope observation.

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
