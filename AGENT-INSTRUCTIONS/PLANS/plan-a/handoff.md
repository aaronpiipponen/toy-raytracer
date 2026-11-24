# Orchestrator handoff

## Next action

Start the A1 implementer. Phase A1 is detailed and passed the orchestrator's acceptance
check, but a coordinator hold is armed against geometry phase B1, so the `phase_subagent`
call is expected to be refused with `COORDINATOR HOLD`; make the call so the stop is
recorded, then wait for the coordinator or plan-b's orchestrator to resume.

## Current state

A1 is detailed in `phases/A1-diffuse-and-metal-materials.md` and accepted. No code is
implemented yet. The user's Z1 survey rewrite and the A2 glass/front-face hand-off are in
`full-plan.md`. `docs-pass.md` records one stale plan mention in `src/vec3.rs` for the
release. Autonomous mode is enabled for routine loop choices only; every product decision
goes to the user. The session is linked to the coordinator; plan-b leads the A1/B1 overlap.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- A held phase's implementer start is still attempted so the refusal records the stop;
  after that, start nothing else until resumed.
