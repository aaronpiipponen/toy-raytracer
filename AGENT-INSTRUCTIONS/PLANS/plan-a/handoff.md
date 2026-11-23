# Orchestrator handoff

## Next action

Run `REVIEW AND ADVANCE` for A2 at `d079bb0`; on advancement take its next-phase handoff and run the
acceptance check before A3 (samples per pixel).

## Current state

A1r done (`b7af7f8`). A2 implemented and committed (`b7af7f8..d079bb0`): glass material, glass
sphere, three tests, 11 tests green, rendered image observed, one review pass with no problems, user
test not required. A1 `not applicable`. A3 and Z1 planned.

Autonomous mode on; standing authorization for phase commits on branch `plan-a`.
Work kind: product work for every phase (gate level `standard`, Z1 `minimal`).

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
