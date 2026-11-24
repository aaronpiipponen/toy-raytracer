# Orchestrator handoff

## Next action

`phase-advancer` is compressing Z1 and closing the plan in the background. When it returns, confirm
the status table shows every phase `done` (B1 `not applicable`), report the plan ready for the
release, and end this plan's work.

## Current state

- Plan implementation complete at `c25007c` and surveyed; the user accepted the survey and the
  rendered scene on 2026-11-24 (recorded in
  `benchmarks/2026-11-24-closing-survey.md`), with no remediation: findings 1 to 6 accepted as
  documented, 7 and 8 left to the release.
- Z1's note and saved returns exist; `docs-pass.md` holds the repository-map release item.
- The release merges plan-a (materials), plan-b (geometry) and plan-c (output), resolves the settled
  conflicts, lands the deferred documentation item, and closes the plan.

## Learned rules

- The leader of a pair writes both resolution phase files, not only its own; correct the partner's
  draft in place rather than creating a second file.
- A dropped phase keeps its id as a `not applicable` row; a resolution phase that absorbs a held
  phase's whole scope takes that scope and later phases depend on it.
- The plan-b branch must never add the material field or `Rc`; that is the release merge's work.
- A resolution phase's own document is not a plan-state edit: status rows, `scope.md`, `done/` and
  compression belong to the advancer, never to the child that writes the settlement.
- Do not edit `AGENT-INSTRUCTIONS/LOCAL/project.md` from one worktree; its map update is a single
  release item in `docs-pass.md` covering all plans' new modules.
