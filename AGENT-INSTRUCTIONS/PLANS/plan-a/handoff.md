# Orchestrator handoff

## Next action

Wait for the user's acceptance of the closing survey or their named changes. On acceptance, send
`REVIEW AND ADVANCE` for Z1 so the advancer compresses the phase into `done/Z1-closing-survey.md` and
flips the row. On named changes, schedule them outside Z1 (a later survey or remediation round). The
plan is not archived without explicit closure authorization.

## Current state

Z1 is the only open phase. The closing survey note is written at
`benchmarks/survey-2026-11-23.md`: 21 cross-checked findings, none fixed, plus out-of-scope
observations and a user-review list. Two renders were byte-identical, the 200x100 image was
inspected; `/tmp/opencode/survey-a.png` and `.ppm` are kept for the user and must be removed after
their reply. Code head `dca633c`, worktree clean, A1r/A2/A3 done, A1 `not applicable`.

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
- Workspace rule: rendering tests check a few pixels of a small image, never a whole image
  (`LOCAL/repo-rules-domain.md:37`).
