# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` to complete Z1:

`REVIEW AND ADVANCE: Review and advance phase Z1 of plan-b. Implementer verification receipt: not applicable; Z1 was executed by the orchestrator (ten read-only ops-scout areas plus one cross-check) and wrote AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/survey-2026-11-23.md; renders verified byte-identical, 20003 lines, header ok, lines 4/7441/10104 as recorded. Relevant implementer comments: none. User-specific test result: the user accepted the survey as it stands and directed that the two coverage findings (F9, F10) must NOT be fixed and no test is to be added; the findings stay recorded in the note.`

After it returns, verify the status table shows the plan complete and report the outcome. The release
follows and closes the plan; do not archive it.

## Current state

- B1 `done` `6eb43e09`; B2r `done` `5204447`; B2 `done` `a5c6665`; B3 `done` `2b4e2b7`.
- Z1: `active`, survey complete, note written, **user accepted 2026-11-23 with no changes**. The user
  explicitly declined the two coverage tests (F9 nearest-hit selection, F10 shape boxes); they stay
  recorded in the note only. This is a settled user decision: no remediation unit, no new test.
- Earlier stale plan statements corrected as bookkeeping (phase test count six, not eight; the
  `full-plan.md` carry-over removed).
- Renders: PPM kept at `/tmp/opencode/survey-a.ppm`, PNG copy at `/tmp/opencode/survey-a.png`.
- Remaining work after Z1: the release, which updates documentation and closes the plan. This session
  does not archive the plan.
- Autonomous mode on; standing authorization for phase commits exists.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp`), never a remembered
  pixel value, when the claim is that the image did not change. A specific pixel probe is acceptable
  only when a diff cannot prove the new behaviour (for example, that a newly added object renders).
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
- Deleting comments that name a plan or a later stage (REPO-RULES §4.13) is legitimate work inside a
  phase that already touches the file; keep it to the comment lines only.
- A `Rc<dyn Trait>` field kills `Copy` and `Debug` on a derived struct; do not add `Debug` as a trait
  supertrait just to keep a derive when no reader prints the value.
- Commits never move between plan branches; parallel plans each implement their own side on their own
  branch and the release merges both into `main`, reconciling textual conflicts by the shared design.
- A phase file that still contains `<angle-bracket placeholders>` is a template, not a detailed
  phase; do not accept it or close a channel on it.
- The partner orchestrator cannot message the leader on its own; when it says it is waiting, re-read
  its phase file rather than waiting for a message.
- Clippy `-D warnings` rejects a literal `x < a || x > b`; a phase that spells it out will be
  implemented as `!(a..=b).contains(&x)`. Do not treat that equivalence as drift.
- The closing survey is orchestrator-dispatched, never inside the implementer or advancer workflow:
  one read-only `ops-scout` per area, then one cross-check `ops-scout`, then the orchestrator writes
  the dated note. It never fixes findings inside that phase.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp`), never a remembered
  pixel value, when the claim is that the image did not change. A specific pixel probe is acceptable
  only when a diff cannot prove the new behaviour (for example, that a newly added object renders).
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
- Deleting comments that name a plan or a later stage (REPO-RULES §4.13) is legitimate work inside a
  phase that already touches the file; keep it to the comment lines only.
- A `Rc<dyn Trait>` field kills `Copy` and `Debug` on a derived struct; do not add `Debug` as a trait
  supertrait just to keep a derive when no reader prints the value.
- Commits never move between plan branches; parallel plans each implement their own side on their own
  branch and the release merges both into `main`, reconciling textual conflicts by the shared design.
- A phase file that still contains `<angle-bracket placeholders>` is a template, not a detailed
  phase; do not accept it or close a channel on it.
- The partner orchestrator cannot message the leader on its own; when it says it is waiting, re-read
  its phase file rather than waiting for a message.
- Clippy `-D warnings` rejects a literal `x < a || x > b`; a phase that spells it out will be
  implemented as `!(a..=b).contains(&x)`. Do not treat that equivalence as drift.
- The closing survey is orchestrator-dispatched, never inside the implementer or advancer workflow:
  one read-only `ops-scout` per area, then one cross-check `ops-scout`, then the orchestrator writes
  the dated note. It never fixes findings inside that phase.
