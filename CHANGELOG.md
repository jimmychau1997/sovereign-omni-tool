# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- GitHub community health files: issue templates (bug report, feature request),
  pull request template, and `SECURITY.md`.
- CI now tests Python 3.10, 3.11, and 3.12 in a matrix and caches pip installs.
- CI security-audit job using `cargo audit`.

### Fixed
- CI workflow and badge now correctly target the `master` default branch.
- `CONTRIBUTING.md` and `pyproject.toml` CHANGELOG URL updated from `main` to
  `master`.

---

## [1.0.0] — 2025-04-07

### Added
- **Portable tool discovery** — `sov` now locates Python tools without hardcoded
  paths. Search order:
  1. `$SOV_TOOLS_PATH` (OS path-separator-delimited list of directories)
  2. `<binary-dir>/tools/`
  3. `<binary-dir>/../tools/`
  4. `<binary-dir>/../share/sov/tools/` (FHS install layout)
  5. `~/.local/share/sov/tools/` (Unix) / `%APPDATA%\sov\tools\` (Windows)
- **`sov list --json`** — machine-readable JSON output of all discovered tools.
- **`sov limit-rate` implementation** — throttles stdin lines to the configured
  rate (lines per second); previously a TODO stub.
- **Rich MCP tool descriptions** — the MCP `tools/list` endpoint now uses
  `arsenal_dump.json` descriptions when available, falling back to a generated
  string.
- **`arsenal_dump.json` auto-discovery** — loaded relative to the binary,
  eliminating the hardcoded absolute path.
- **Unit tests** covering tool discovery, filtering, path resolution, and
  deduplication.
- `LICENSE` (MIT), `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`.
- GitHub Actions CI workflow (`ci.yml`): build, test, fmt, clippy, and Python
  lint on every push/PR.
- `pyproject.toml` for the Python MCP server.
- `tools/README.md` — explains how to add Python tool scripts.

### Changed
- `Cargo.toml` version bumped from `0.1.0` to `1.0.0`.
- `Cargo.toml` enriched with `description`, `license`, `repository`,
  `keywords`, and `categories` for crates.io readiness.
- `sov_mcp.py` binary and dump-path discovery is now automatic (env vars
  `SOV_BIN` / `SOV_DUMP_PATH`, then `$PATH`, then build-output paths).
- `sov_mcp.py` now gracefully handles a missing `sov` binary with a clear
  error message.
- Improved `sov list` output: shows total count and helpful instructions when
  no tools are found.
- Improved `sov <unknown>` error message: directs users to `sov list`.

### Removed
- Hardcoded paths (`/home/c/...`) from `src/main.rs` and `sov_mcp.py`.
