# Orchestrator handoff

## Next action

Run `REVIEW AND ADVANCE` for A1r at `b7af7f8a0ad52671c92b1ca62d0825feb4fbb13f`; on advancement,
take its next-phase handoff and run the acceptance check before detailing/implementing A2.

## Current state

A1r implemented and committed (`e180476..b7af7f8`), fast gate and rendered-image gate green, one
review pass with no findings, user test not required. A1 `not applicable` (carried by A1r). A2
(glass), A3 (samples) and Z1 planned. The coordinator may re-examine the plan-c overlap on the
implemented A1r; that is outside this phase and must not change it.

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
