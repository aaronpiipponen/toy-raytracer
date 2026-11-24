# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL RESOLUTION` for pair plan-a-A1+plan-b-B1 and
plan-a's held phase A1. Then run the acceptance check on plan-a's resolution phase file
and on plan-b's `B1r`, and send plan-b a `channel_notify` when plan-a's file is ready.

## Current state

The A1+B1 shared design is settled in `PLANS/shared/plan-a-A1+plan-b-B1.md` and both
branches must reach it on their own branch, with the release resolving the textual
conflicts. plan-b has `phases/B1r-shared-hit-record-shape.md` detailed and accepted by
its orchestrator; B1 is marked not applicable. A1 is unimplemented; a coordinator hold
blocks it until the pair resolves. The user's ground-plane and triangle materials are
settled (matte; mid-grey ground, dull green triangle). Autonomous mode is enabled for
routine loop choices only; every product decision goes to the user. This session is
linked to the coordinator and is the pair's partner; plan-b leads and closes.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- A held phase's implementer start is still attempted so the refusal records the stop;
  after that, start nothing else until resumed.
- The shared design is authoritative for the resolution; plan-a's scope excludes face
  sides and texture coordinates, so the shared record's geometry fields are plan-b's
  branch work and the release merge's, never plan-a's.
