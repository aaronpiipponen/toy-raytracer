# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL RESOLUTION` for pair plan-a-A1+plan-b-B2 to write
plan-a's `A1r3` and plan-b's `B2r` phase files from the agreed shared design. Then run my
acceptance check on both, tell plan-b they are final, and wait for `RESOLVED`.

## Current state

The A1+B2 design is agreed and written to `PLANS/shared/plan-a-A1+plan-b-B2.md`: one
composed scene (matte centre sphere, matte mid-grey ground plane, matte dull-green triangle,
metal sphere), both new shapes gain a material field and pass it to the one hit-record
constructor, the scene list keeps the borrow-based nearest record and adds B3's box test,
and both resolution phases are documentation-only. Per the user's rule for this workspace,
the leader (this session) writes both resolution phase files. A1 is held until the pair
resolves; A1r2 is done and A1's detail is current at `e180476`. The docs-pass note about the
stale `Vec3::cross` comment is dropped because plan-b's B2 deletes that comment.
Autonomous mode covers routine loop choices only; every product decision goes to the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; a partner never starts the held
  phase before `RESOLVED`; a closed pair is not reopened for wording alone.
- The leader of a pair writes both resolution phase files; the partner checks them.
- A child's claimed correction must be checked against the file.
- A1 must not duplicate the geometry plan's face side, texture coordinates, constructor or
  bounding boxes.
- Resolution ids never reuse an existing id: the third resolution of A1 is `A1r3`.
