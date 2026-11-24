# Orchestrator handoff

## Next action

None for this plan's serial loop. Every phase is done; the plan is ready for the release
orchestrator, which owns closure and archiving. If the user reopens work, read
`full-plan.md` and the relevant `done/` contract first.

## Current state

Plan complete at product head `a7ce19d` on branch `plan-a`: materials (A1), glass (A2),
jittered sampling (A3), two survey fixes (R1), and the closing survey with the user's
dated acceptance (Z1). The status table's eight rows all read `done`; `phases/` is empty
because each phase was compressed into `done/`. The three shared designs with the
geometry and output plans are settled, and each plan's branch stays buildable alone; the
release merges them and composes the hit record, the render loop, the scene and the output
per those files. Deferred to the release: `Vec3::cross`'s dead code and plan-naming comment
(resolved by the geometry plan's plane phase and the merge) and the repository-map entries
for the other plans' modules (`docs-pass.md`). The user optionally still owes
`/workflow-reload` after the `project.md` map edits.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; a partner never starts the held
  phase before `RESOLVED`; a closed pair is not reopened for wording alone - except a line
  the user explicitly approved correcting, which is changed together with the code it
  describes.
- The leader of a pair writes both resolution phase files; the partner checks them.
- A child's claimed correction must be checked against the file.
- A1 must not duplicate the geometry plan's face side, texture coordinates, constructor or
  bounding boxes.
- Resolution ids never reuse an existing id: a future resolution of A1 is `A1r4`.
- A documentation-only phase must not be left in a shell-check loop: send one narrow
  follow-up to re-read the settlement against the shared file and return the receipt, and
  never let a paper phase block the next product phase.
- Never accept a jitter formula that samples outside its own pixel: the offset is centred
  (`+ next_f64() - 0.5`), so the image keeps the previous revision's geometry.
- The closing survey is orchestrator-run: no implementer writes the note and Z1 fixes
  nothing it finds; accepted findings become a separate remediation unit.
- A later contract may supersede a frozen one; record the supersession in the new contract
  and leave the old file frozen and unedited.
- Verbatim scout returns go into the returns file as each return arrives; the survey note
  is written only from complete saved returns.
