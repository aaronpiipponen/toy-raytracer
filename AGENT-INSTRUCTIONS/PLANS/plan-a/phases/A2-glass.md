## Phase A2 - Glass

**Status:** planned.
**Depends on:** <real completed phase/contract or none>.
**Scope:** <exact boundary of this phase>.
**Gate level:** <minimal | standard | full | security> - <why this level fits the risk>.
**Simplicity statement:** <what is deliberately not handled (raises a clear error); legacy/defensive code deleted>.

**Current state (verified):**
- `<path>` - <real symbol/signature/behavior>
- `<path>` - <real symbol/signature/behavior>

**Read first (do not invent):**
- `<path>` - `<exact signature or table/config shape>` and why it matters.
- `<path>` - `<exact signature or table/config shape>` and why it matters.

**Build plan:**
1. `<path>`, `<symbol>` - exact edit, including before -> after signature if changed.
2. `<path>`, caller/wiring/import - exact ripple from step 1.
3. `<path>`, tests - exact test edits/additions.
4. `<docs path>` - exact page edit (create, update or delete).

**Callers / wiring to update:**
- `<path:line or symbol>` - <required update>

**Config / schema / migrations:**
- <exact config keys or migration filenames, or "none">
- Retained records and their named consumer/owner: none, or exact justified inventory.

**Touched files:**
<!-- touched-files:start -->
- `<path>` - <created | changed | deleted>: <one line on what changes>
- Adds: <migration filenames, generated files, configuration keys, or "nothing">
<!-- touched-files:end -->

**Rules / MUST NOT:**
- <permissions, no duplicate path, no fake fallback, no scope creep>

**Tests:**
- New/changed/deleted: `<test file>` - `<test_name>`: <behavior> (maximum 3 new test functions, not a quota; adapt existing tests first; zero new when unjustified; no line/count gates; REPO-RULES §3a)
- Fast gate: <targeted tests + lint/format/diff checks that apply>.
- Conditional gates: <only touched seams; state why each is required or `none`>.
- Slow gate (>2 minutes): <command, reason, sole execution owner, blocking/nonblocking; or `none`>.

**User test (acceptance):**
- **Executor:** <not required | agent | user - use user only when named capability is unavailable>.
- Use `Not required - automated phase gates prove acceptance` when no distinct manual/runtime check adds evidence.
- <numbered, concrete steps and exact result to observe>
- For UI: which page, what to click/type, expected on-screen result; use user executor only
  when correctness requires subjective visual/interaction judgment or unavailable access.
- For backend-only: exact command or request and expected output/response; agent normally runs it.
- Keep it short and unambiguous; this is the gate that catches breakage before next phase.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] <verifiable item>
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports functions added/deleted. Any line counts are observations, not gates.
- [ ] Fast phase gate and justified affected conditional gates from `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 pass; slow gates ran once by named owner.
- [ ] The named `docs/` pages are updated in this change, none is stale or wrong, and none refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; user test has passing evidence only when separate acceptance was required.
- [ ] On passing evidence: compress this file into `done/A2-<title>.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** <observable end state>.
