# Orchestrator handoff

## Next action

Run `REVIEW AND ADVANCE` for Z1 with the user's acceptance so the advancer compresses
`phases/Z1-closing-survey-and-user-acceptance.md` into `done/Z1-closing-survey.md` and flips its row.
Then report the completed plan; the release follows and closes/archives it.

## Current state

The user accepted the closing survey (`benchmarks/survey-2026-11-23.md`) as it stands and scheduled
no changes; findings stay recorded. All implementation phases are done (A1r, A2, A3); A1 is
`not applicable`. Code head `dca633c`, worktree clean. Remove `/tmp/opencode/survey-a.png` and
`survey-a.ppm` after their reply.

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
- Plan closure and archiving belong to the release; I never archive the plan myself.
