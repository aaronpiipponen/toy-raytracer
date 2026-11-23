# Orchestrator handoff

## Next action

None. The plan's work is complete and the user accepted the closing survey; the release follows and
closes and archives the plan. Do not archive it here.

## Current state

Every status row is `done` or `not applicable`: A1r, A2, A3 and Z1 done, A1 `not applicable` (carried
by A1r). Code head `dca633c`, worktree clean, branch `plan-a`. Survey note at
`benchmarks/survey-2026-11-23.md` with 21 findings recorded and none scheduled, as the user directed.
Scratch renders in `/tmp/opencode` removed.

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
  repository, and are removed once the test is accepted.
- The closing survey is mine to run: parallel area scouts, one cross-check scout, then I write the
  dated note; the plan closes only on the user's acceptance.
- Workspace rule: rendering tests check a few pixels of a small image, never a whole image
  (`LOCAL/repo-rules-domain.md:37`).
- Plan closure and archiving belong to the release; I never archive the plan myself.
