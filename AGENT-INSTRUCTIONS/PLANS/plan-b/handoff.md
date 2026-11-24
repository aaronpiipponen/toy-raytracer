# Orchestrator handoff

## Next action

`phase-advancer` is reviewing and advancing B3 in the background. When it returns, read its Z1
handoff: Z1 is the closing survey (one `ops-scout` per area in parallel, then one cross-checker, then
the orchestrator writes the dated note in `benchmarks/`) plus the user's acceptance of the rendered
scene. Run the orchestrator acceptance check on Z1's detail before any work starts, and confirm with
the user the survey areas before dispatching scouts.

## Current state

- B1r, B2r, B2 done; B3 implemented and reviewed clean at `c25007c` (13 tests, clippy, fmt,
  `git diff --check`, byte-identical render; 3 tests added). Earlier orchestrator commit `281dda8`
  removed a stale plan comment on `mod rng`.
- Branch plan-b has no material field and no `Rc`: awaiting the release merge, which adds materials,
  the borrowed nearest record and box composition, per `PLANS/shared/plan-a-A1+plan-b-B1.md` and
  `PLANS/shared/plan-a-A1+plan-b-B2.md`.
- Open small item for the user: `AGENT-INSTRUCTIONS/LOCAL/project.md`'s repository map omits
  `plane.rs`, `triangle.rs` and (after B3) `aabb.rs`; my proposed exact wording was sent. Editing it
  needs the user's approval and a `/workflow-reload` decision.
- Autonomous mode: small, easily undoable, behaviour-neutral choices only; `autonomous-decisions.md`
  when the first is recorded. Product decisions still go to the user.

## Learned rules

- The leader of a pair writes both resolution phase files, not only its own; correct the partner's
  draft in place rather than creating a second file.
- A dropped phase keeps its id as a `not applicable` row; a resolution phase that absorbs a held
  phase's whole scope takes that scope and later phases depend on it.
- The plan-b branch must never add the material field or `Rc`; that is the release merge's work.
- A resolution phase's own document is not a plan-state edit: status rows, `scope.md`, `done/` and
  compression belong to the advancer, never to the child that writes the settlement.
