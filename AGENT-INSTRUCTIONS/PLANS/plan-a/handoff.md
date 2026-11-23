# Orchestrator handoff

## Next action

Run `REVIEW AND ADVANCE` for A3 at `dca633c`; on advancement take its Z1 handoff. Z1 is the closing
survey phase: I dispatch one read-only `ops-scout` per area in parallel, then one cross-check scout,
then write the dated note into `benchmarks/` myself, and the user accepts before the plan closes.

## Current state

A1r, A2 done. A3 implemented and committed (`d079bb0..dca633c`): `SAMPLES = 16`,
`render(width, height, samples)` averaging jittered rays, one new test, 12 tests green, rendered
image observed smooth, review clean, user test not required. Z1 planned.

Autonomous mode on; standing authorization for phase commits on branch `plan-a`.
Work kind: product work (Z1 `minimal`).

## Learned rules

- A3 makes the sample count a parameter of `render` with 16 as the internal default; the output plan
  later exposes `--samples`.
- No cross-branch code transfer and no branch merges another; a shared design only aligns what each
  side writes on its own branch, and the release merges both into main.
- A resolution phase carries the held phase's work; the held row becomes `not applicable` rather
  than staying as an empty planned phase.
- A coordinator hold armed after an implementer has already started is not valid; resume that same
  implementer session rather than restarting the phase.
- Render artifacts left in `/tmp/opencode` are reused by a phase's user test, never written into the
  repository.
- The closing survey is mine to run: parallel area scouts, one cross-check scout, then I write the
  dated note; the plan closes only on the user's acceptance.
