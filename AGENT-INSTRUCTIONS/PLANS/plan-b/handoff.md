# Orchestrator handoff

## Next action

`phase-implementer` is implementing the active phase B1r in the background. When it returns a
complete receipt (fresh reviewer, fixes and fresh passes finished), read the SHA, receipt, comments
and user-test declaration, then spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for B1r.

## Current state

- Pair `plan-a-A1+plan-b-B1` is closed; both holds are released and the channel is gone.
- `phases/B1r-shared-hit-record-shape.md` - detailed and accepted; B1r is the active phase.
- B1 is `not applicable` (absorbed by B1r); B2 depends on B1r; Z1 depends on B1r + B2 + B3.
- Shared design `PLANS/shared/plan-a-A1+plan-b-B1.md` is final: geometry fields and the six-argument
  constructor on this branch; material field, `Rc`, the seven-argument constructor and the dropped
  `Copy`/`Debug` in the release merge; ground plane mid-grey, triangle dull green.
- Autonomous mode is enabled for small, easily undoable, behaviour-neutral choices only;
  `autonomous-decisions.md` is created when the first such decision is recorded. Product decisions
  still go to the user.

## Learned rules

- The leader of a pair writes both resolution phase files, not only its own; correct the partner's
  draft in place rather than creating a second file.
- A dropped phase keeps its id as a `not applicable` row; a resolution phase that absorbs a held
  phase's whole scope takes that scope and later phases depend on it.
- The plan-b branch must never add the material field or `Rc`; that is the release merge's work.
