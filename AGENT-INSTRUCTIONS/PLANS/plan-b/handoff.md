# Orchestrator handoff

## Next action

Wait for the user's Z1 acceptance. The pair `plan-a-A3+plan-b-Z1` is RESOLVED and the Z1 resolution
record is in the phase file; the survey is complete and the survey verdict is "no confirmed clear
defect". On the user's yes: start a fresh advancer with `REVIEW AND ADVANCE: Review and advance phase
Z1 of plan-b`, passing the survey note path (`benchmarks/closing-survey-2026-11-24.md`), the render
evidence (md5 `13b3bc0a3cf5c41df684e179c1d3e93d`) and the user's acceptance wording; then remove
`/tmp/opencode/plan-b-acceptance.ppm`, `plan-b-acceptance.png` and `ppm2png.py` and stop - release and
closure are not this session's work. On a named blocker, open the remediation unit the user asks for.

## Current state

B1, B2 and B3 are done; head `f85a183`. Z1 is active and its survey is complete: four area returns and
the cross-check return are saved in `benchmarks/returns/`, and the dated note
`benchmarks/closing-survey-2026-11-24.md` is written. Verdict: no confirmed clear defect; one
shared-tree documentation item (repository map names `material.rs`, which this branch lacks; owner is
the release merge), several nits, and a user-review list. Acceptance render:
`/tmp/opencode/plan-b-acceptance.png` (PPM md5 `13b3bc0a3cf5c41df684e179c1d3e93d`, same as the B3
before/after render); keep it until the user answers, then remove it with the PPM and
`/tmp/opencode/ppm2png.py`.

Open items to carry:
- Release must ask the user which material the scene triangle gets
  (`PLANS/shared/plan-a-A2+plan-b-B2.md`).
- `/workflow-reload` for the shared `project.md` edits is still a question for the user.

## Learned rules

- The user enabled autonomous mode for the routine loop: settle small, easily undoable,
  behavior-neutral choices myself and record each with its reason and how to undo it in
  `autonomous-decisions.md`. Ask the user only for product decisions, a user test or acceptance, and
  plan closure.
- Scope authorizes phase commits on branch `plan-b`: commit whenever useful, never squash, never to
  `main`.
- Test-cap hygiene is part of my acceptance check: merge near-duplicate test functions that share one
  fixture instead of spending the cap.
- Parallel-plan splits: each branch stays compiling on its own; obligations only the release merge can
  satisfy live in the shared design file, never as placeholders in code.
- A shared instruction-tree line that is true for another plan's worktree is reported, not "fixed"
  from this branch.
