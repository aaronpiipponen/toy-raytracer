# Orchestrator handoff

## Next action

Implement A3 from `phases/A3-samples-per-pixel.md`; on its receipt run `REVIEW AND ADVANCE` for A3,
then Z1 (closing survey).

## Current state

A1r and A2 done (`b7af7f8`, `d079bb0`). A3 active and detailed (passed the orchestrator acceptance
check): `SAMPLES = 16`, `render(width, height, samples)` averaging jittered rays from the one seeded
generator, one new exact-average test, two test calls adapted. Z1 remains planned.

Autonomous mode on; standing authorization for phase commits on branch `plan-a`.
Work kind: product work (A3 `standard`, Z1 `minimal`).

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
