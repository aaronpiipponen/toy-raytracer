# Orchestrator handoff

## Next action

`phase-advancer` is reviewing and advancing B1r in the background. When it returns, read its
next-phase handoff, detail B2 fresh (`DETAIL ACTIVE`) only after the status table shows it, then run
the orchestrator acceptance check and start B2's implementer.

## Current state

- B1r implemented and reviewed clean at `f0f11e2` (receipt: build, 7 tests, clippy, fmt,
  `git diff --check`, byte-identical PPM; 2 tests added, 1 extended).
- Reviewer notes: `docs/` needed no change; the constructor doc comment in the phase snippet was
  written as a noun phrase while the code uses the imperative form the documentation style guide
  requires. Compression/status flip belong to the advancer.
- Pair `plan-a-A1+plan-b-B1` is closed; shared design final.
- B1 is `not applicable`; B2 depends on B1r; Z1 depends on B1r + B2 + B3.
- Autonomous mode is enabled for small, easily undoable, behaviour-neutral choices only;
  `autonomous-decisions.md` is created when the first such decision is recorded. Product decisions
  still go to the user.

## Learned rules

- The leader of a pair writes both resolution phase files, not only its own; correct the partner's
  draft in place rather than creating a second file.
- A dropped phase keeps its id as a `not applicable` row; a resolution phase that absorbs a held
  phase's whole scope takes that scope and later phases depend on it.
- The plan-b branch must never add the material field or `Rc`; that is the release merge's work.
