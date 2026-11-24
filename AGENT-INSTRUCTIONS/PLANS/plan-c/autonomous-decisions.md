# Autonomous decisions

Orchestrator-only record. Children never read or edit this file.

## 2026-11-24 - Either output extension or explicit format

- **Decision:** an explicit `--format` wins over the output file's extension; an extension that is
  neither `.png` nor `.ppm` (with no explicit format) is a bad value, so it prints the usage line and
  exits non-zero.
- **Reason:** requirement 12 names both an explicit option and an extension-derived default without
  saying which wins; the flag the user typed is the more specific statement of intent, and failing on
  an unknown extension keeps one format choice instead of guessing.
- **Undo:** state the precedence in `docs/requirements.md` item 12 and re-detail C1; the change is one
  branch in the option resolver.
