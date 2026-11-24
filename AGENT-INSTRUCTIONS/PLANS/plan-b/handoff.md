# Orchestrator handoff

## Next action

Executing the closing survey Z1 (the orchestrator runs it): six read-only `ops-scout` area returns in
parallel are saved to `survey-returns-2026-11-24.md`, then one fresh cross-checker, then the note at
`benchmarks/2026-11-24-closing-survey.md`, then the acceptance render, then the user's acceptance.

## Current state

- B1r, B2r, B2, B3 done; HEAD `c25007c`; survey base `e180476`; seven changed files match the phase
  document exactly.
- `docs-pass.md` holds the release item for the repository map; the user approved deferring it and
  forbade editing `project.md` or running `/workflow-reload` now.
- Z1 writes only plan records; nothing is fixed in it; its move-on gate is the user's acceptance of
  the note and the rendered scene.
- Autonomous mode: small, easily undoable, behaviour-neutral choices only. Product decisions and the
  acceptance itself are the user's.

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
