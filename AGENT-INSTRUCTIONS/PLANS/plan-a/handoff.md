# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for A1r2 with the implementer's
receipt. Then attempt the A1 implementer: A1 is held against plan-b's B2 and I lead that
settlement, so expect `COORDINATOR HOLD`, record the stop and wait.

## Current state

A1r2's implementer returned a passing documentation-only receipt at unchanged head
`e180476`; it fixed two plan-text inaccuracies in the phase file (stale plan-c line
references and the progress-call placement). One drift is left alone deliberately: the
shared render file's rule 7 calls both progress calls "inside the row loop" while plan-c's
trailing `eprintln!` runs after it; the operative rule "the merge keeps plan-c's calls
unchanged" is unaffected and the phase records the true placement, so I did not reopen a
closed pair for wording. `done/A1r` has one frozen sentence that no longer matches the
table; done contracts are frozen and it stays. The A1/B2 hold is armed with plan-a leading.
The user's ground-plane and triangle materials are settled. Autonomous mode covers routine
loop choices only; every product decision goes to the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; a partner never starts the held
  phase before `RESOLVED`, and a closed pair is not reopened for wording that does not
  change the merged code.
- A child's claimed correction must be checked against the file.
- A1 must not duplicate the geometry plan's face side, texture coordinates or constructor.
- Resolution ids never reuse an existing id: the third resolution of A1 is `A1r3`.
