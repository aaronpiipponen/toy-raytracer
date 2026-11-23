"""Publishing data for one checkout, loaded by the workflow exporter.

Copy this file to ``AGENT-INSTRUCTIONS/PUBLISHING/selection.py`` and replace the values with that
checkout's own selection. The lists below are an illustration, not a required layout.
``export.py`` beside it reads these names and nothing else; it carries no
paths of its own, so the same exporter publishes any checkout.

Source paths are relative to the checkout. Destinations are relative to ``TARGET_ROOT``.
"""

from __future__ import annotations

from pathlib import Path

# The public checkout this selection is mirrored into.
TARGET_ROOT: Path = Path.home() / "projects" / "public-checkout"

# Account homes that exist inside the served images, not on a host. Paths under them are not
# treated as a leaked host path.
CONTAINER_HOME_ACCOUNTS: tuple[bytes, ...] = ()

# Shared documentation authored here and published verbatim: every regular file under this root
# is mirrored to the same path, except the exclusions. Everything else is listed in FILE_MAP.
SHARED_DOCS_ROOT: str = "docs"
SHARED_DOCS_EXCLUDED: tuple[str, ...] = ("docs/private", "docs/USER-OWNED-TODO.md")

# (source, destination) for every exported file outside the shared documentation root. These two
# are an illustration; list the checkout's own.
FILE_MAP: tuple[tuple[str, str], ...] = (
    ("LICENSE", "LICENSE"),
    ("README.md", "README.md"),
)

# Explicit writes into a protected tree, checked exactly like FILE_MAP. Use these for deployment
# assets that live in the public checkout but are authored here.
PROTECTED_COPIES: tuple[tuple[str, str], ...] = ()

# Paths that exist only in the public checkout and must never be copied into or removed from it.
PUBLIC_ONLY: tuple[str, ...] = (".git", ".github", "config", ".env")

# Directory names the target cleanup may remove at any depth.
PRUNE_DIR_NAMES: frozenset[str] = frozenset({"__pycache__", "node_modules", ".cache"})

# Text that must never appear in an exported file: this checkout's private trees and tooling.
PRIVATE_MARKER_PATTERN: bytes = (
    rb"(?:docs/private|AGENT-INSTRUCTIONS|USER-OWNED-TODO|AGENTS\.md)"
)

# Exported shared Markdown may not link into these; private pages may not link into the subset in
# PRIVATE_DOC_LINK_FORBIDDEN. Private-to-private and private-to-shared links stay allowed.
SHARED_DOC_LINK_FORBIDDEN: tuple[str, ...] = (
    "docs/private",
    "docs/USER-OWNED-TODO.md",
    "AGENTS.md",
    "AGENT-INSTRUCTIONS",
)
PRIVATE_DOC_LINK_FORBIDDEN: tuple[str, ...] = (
    "docs/USER-OWNED-TODO.md",
    "AGENTS.md",
    "AGENT-INSTRUCTIONS",
)

# A leaked copy of these in the target stops the run before any mutation.
TARGET_GUARDED: tuple[str, ...] = (
    "docs/private",
    "docs/USER-OWNED-TODO.md",
    "AGENTS.md",
    "AGENT-INSTRUCTIONS",
)
