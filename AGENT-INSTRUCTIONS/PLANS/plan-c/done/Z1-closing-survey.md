## Z1 - Closing survey and user acceptance

**Status:** done.
**Depends on:** C1 and C3, whose frozen contracts are in `done/C1-png-writer-and-command-line.md`
and `done/C3-progress-line.md`, and C1's survey remediation in
`done/C1-survey-remediation.md`.
**Gate level:** minimal.

**Frozen record:**
- The plan orchestrator ran the survey itself with read-only scouts. No `phase-implementer`
  session ran and no code, test, configuration, README or `docs/` file changed in this phase.
- The survey note is
  `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/closing-survey-2026-11-24.md`. It is the only copy
  of the survey and its consumer is the user's acceptance decision and the release's review of
  `docs-pass.md`. The saved area returns are in
  `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/returns/`.
- The note surveys the plan's whole work, `e1293ef..0ad0f2b`, with four independent read-only
  `ops-scout` area returns (the PNG writer, the command line and settings, the render loop and
  progress line, and tests and documentation) and one independent read-only `ops-scout`
  cross-check of every finding claim by claim.
- The survey confirmed six small findings: four test-proof gaps and one parser duplication in
  C1's work, and one stale release-deferral wording in the plan record. C1's survey remediation at
  `a0c585b` fixes the five C1 findings, and the plan orchestrator fixed the plan-record wording in
  `docs-pass.md`. The note's rejected and corrected supporting points stay recorded so a later
  reader does not reopen them. No material code defect was found and no finding needed a phase of
  its own.
- The note's intentionally-unfixed list is part of the accepted record: the multi-block stored
  deflate path has no dedicated test because C1's three-test budget is used and the 400 by 200
  acceptance render exercises four blocks, the test decoder accepts only `IHDR`, `IDAT` and
  `IEND`, matching the writer, and the README does not name the PPM format. No fourth test is
  authorized.
- The acceptance render at `a0c585b` is a 400 by 200, 16-sample PNG that opens showing the sphere
  over the sky and the ground plane. Standard output is empty and the progress line updates in
  place on standard error, ending `100%` plus one newline. The fast gate passed at `a0c585b` and no
  code changed afterward.
- On 2026-11-24 the user read the note, saw the final render and answered "accept all", accepting
  the note and the rendered result as written, including the intentionally-unfixed list. No
  follow-up solution phase is required.
- `docs-pass.md`, the shared design
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` and
  `AGENT-INSTRUCTIONS/LOCAL/gates.md` are unchanged by this phase. The plan is ready for the
  release, which closes it. It is neither closed nor archived here.
