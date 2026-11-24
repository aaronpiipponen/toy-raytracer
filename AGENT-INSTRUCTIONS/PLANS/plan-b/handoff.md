# Orchestrator handoff

## Next action

Waiting on the user's acceptance of the Z1 closing survey note
(`AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/2026-11-24-closing-survey.md`) and the rendered scene
(`/tmp/opencode/z1-accept.ppm`). On acceptance, record the decision in the note's **User decision**
section and spawn a fresh `phase-advancer` to compress Z1 and close the plan. If the user names a
defect, add it to the note's **For the user** list and decide on a remediation round without fixing it
inside Z1.

## Current state

- Plan implementation complete at `c25007c`: B1r, B2r, B2 and B3 done; all 13 tests, build, clippy,
  fmt clean; rendered image unchanged since B2 and byte-identical across B3 and Z1.
- Z1 survey done: six area returns saved, one independent cross-check done, dated note written; no
  finding was fixed.
- `docs-pass.md` holds the repository-map release item; the user forbade editing `project.md` or
  running `/workflow-reload` from this worktree.
- After user acceptance the release follows: it merges plan-a (materials) and plan-c (output) with
  this branch, resolves the settled conflicts, lands the deferred documentation item, and closes the
  plan.

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
