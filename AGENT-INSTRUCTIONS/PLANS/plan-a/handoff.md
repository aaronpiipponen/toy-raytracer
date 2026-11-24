# Orchestrator handoff

## Next action

Run `phase-advancer` in DETAIL ACTIVE mode: re-sequence so A2 is blocked (coordinator settlement of the
shared face-side shape pending) and A3 is active, then detail A3. After DETAIL READY, run the
acceptance check on the A3 phase file and start A3's implementer.

## Current state

A1 is done and verified on branch `plan-a` (base `e180476`, tested head `de1cf31`); its frozen contract
is `done/A1-diffuse-and-metal-materials.md`. A2 cannot be detailed until the coordinator settles the
shared face-side shape with the geometry plan (plan-b B1 overlaps A1's `HitRecord` and `Sphere`), so
A3 (samples per pixel) runs next. `docs-pass.md` holds one item for the release.

## Learned rules
