# Area 2 return - the command line and settings (raw ops-scout return)

## Answer

The command-line and settings area satisfies the frozen C1 contract and requirements item 12 on every behaviour I could check statically: one `Settings` value is resolved before rendering (`src/main.rs:150-157`), an explicit `--format` beats the extension (`src/main.rs:172-175`), unknown options/bad values/unrecognised format or extension all exit 2 with an error line plus `USAGE` (`src/main.rs:200-204`), a writer/path failure exits 1 with one error line (`src/main.rs:206-209`), stdout stays empty (image goes through `fs::write`, progress through `eprint!`), and defaults live only in the four constants. The only defect is structural: the five option names are written twice and guarded by an `unreachable!` branch (`src/main.rs:119`, `126-146`), which is avoidable duplication rather than a behaviour bug. No invented fallback, extra option, dead field or second output path exists.

## CONFIRMED FINDINGS

**1. Duplicate option vocabulary plus an unreachable branch — class: unnecessary complexity / duplication (minor, no behavioural effect)**
- `src/main.rs:119` — first match lists the five option spellings solely to reject anything else.
- `src/main.rs:126-145` — second match repeats the same five spellings to do the work.
- `src/main.rs:146` — `_ => unreachable!("only the five options reach this match")` is a branch the first match already makes impossible; it exists only to satisfy `&str` exhaustiveness.
- The five-name list is maintained in two places, so adding/renaming an option in one arm would make the other report "unknown option" or panic. A single match (each arm fetching its own value, `other => Err(unknown option)`) removes both the duplication and the unreachable arm. Low severity; behaviour is correct.

## OUT-OF-SCOPE OBSERVATIONS

- `#[allow(dead_code)] mod rng;` at `src/main.rs:7-9` predates this plan (present at `e1293ef`). Design rule 4 (`AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md:33-34`) forbids `allow(dead_code)` unless it "papers over plumbing"; this is a forward placeholder for the materials plan, not plumbing, but it is worth the orchestrator confirming the cross-plan intent.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md:36` still carries "Until the command-line phase lands, `cargo run --release > …render.ppm` instead." That sentence is now false on branch `plan-c`, but it is *deliberately* deferred: rule 8 at `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md:58-61` and the entry in `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md:7-9`. Not a drift defect.
- `tests/cli.rs:158-250` embeds a full PNG chunk decoder, stored-deflate inflater, CRC-32 and Adler-32 to test the writer — likely area 1's concern; large for one assertion.

## Coverage notes (not defects; in-scope tests)

- `tests/cli.rs:88-107` checks only a non-zero exit, not the contract's status 2, and never asserts the usage line text or that stdout is empty; the defaults (400×200, 16 samples, `render.png`) are tested nowhere. The code implements these (`src/main.rs:23-26, 200-204`), so this is a test gap, not drift. NOT VERIFIED by execution because a scout may not build or run tests; judged statically.

## AREA VERDICT

Clean behaviourally and against the C1 contract. One minor structural problem: the option list is duplicated across two matches and guarded by an `unreachable!` (`src/main.rs:119`, `126-146`). `docs/requirements.md` is unchanged from `e1293ef` (empty diff) and matches the implementation.
