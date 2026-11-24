# Orchestrator handoff

## Next action

Plan complete and ready for the release; no orchestrator work remains. The release merges plan-a,
plan-b and plan-c, resolves the settled conflicts, lands the deferred documentation item and closes
and archives the plan. Do not close or archive it from here.

## Current state

- Every phase is `done` except B1 (`not applicable`); the detailed phase files are retired into
  `done/`. HEAD `c25007c`, working tree clean; 13 tests, build, clippy and fmt green; the rendered
  image is byte-identical across B3 and Z1 (md5 `4e0a5f55...`).
- The user accepted the closing survey and the rendered scene on 2026-11-24 with no remediation;
  findings 1 to 6 are documented, 7 and 8 are release bookkeeping.
- Release inputs: shared designs `PLANS/shared/plan-a-A1+plan-b-B1.md`,
  `plan-a-A1+plan-b-B2.md` and `plan-a-A1+plan-c-C3.md`; merge obligations in
  `done/B2r-shared-scene-settlement.md`, `done/B3-bounding-boxes.md` and the other plans' records
  (material arguments, borrowed nearest record plus the box test, the four-material scene); and
  `docs-pass.md` for the repository map (`aabb.rs`, `plane.rs`, `triangle.rs`, `png.rs`). Also
  reconcile plan-c's stale `docs-pass.md` entries for the resolved `main.rs` rng and `hittable.rs`
  point comments.
- Survey evidence: `benchmarks/2026-11-24-closing-survey.md` and
  `survey-returns-2026-11-24.md`.

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
