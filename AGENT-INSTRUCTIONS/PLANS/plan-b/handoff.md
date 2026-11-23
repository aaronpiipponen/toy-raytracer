# Orchestrator handoff

## Next action

Phase B1 is running in session `ses_df9251908ffebHeRBWaY5yy292`. On its receipt, advance the plan:
judge B1, freeze it into `done/`, flip its row, and detail B2.

## Current state

- `B1r` done: `done/B1r-face-side-and-texture-coordinates.md`. `HitRecord` carries `front_face`,
  `u`, `v` through the one `HitRecord::new`; `Sphere` fills latitude/longitude. Gate green at
  `e5859d4`.
- `B1` active: tiny closeout of the same row (delete the two plan-naming comments in
  `src/main.rs:6` and `src/vec3.rs:21`, add `assert!(hit.front_face)` to the existing front-face
  test). No new test functions, no plan-a symbols.
- The merged tree (Clone-only derive, `material` field, helper's material parameter, `as_ref` in
  `HittableList`) is the coordinator's merge step per `shared-plan-a+plan-b.md`; no plan-b branch
  file may name plan-a's symbols.

## Learned rules

- Pair overlap with plan-a was settled in `shared-plan-a+plan-b.md`; both copies must stay
  identical.
- `full-plan.md` "Starting state" and the scoped `B1` paragraph still describe the pre-`B1r` tree;
  correct them when `B1` closes.
