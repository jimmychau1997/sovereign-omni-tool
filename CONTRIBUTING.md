# Contributing to Sovereign Omni-Tool

Thank you for your interest in contributing! All contributions — bug reports,
feature requests, documentation improvements, and pull requests — are warmly
welcomed.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Project Structure](#project-structure)
4. [Development Workflow](#development-workflow)
5. [Adding a New Tool](#adding-a-new-tool)
6. [Commit Messages](#commit-messages)
7. [Pull Request Guidelines](#pull-request-guidelines)
8. [Reporting Bugs](#reporting-bugs)
9. [Requesting Features](#requesting-features)

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you agree to abide by its terms.

---

## Getting Started

### Prerequisites

| Requirement | Minimum version |
|-------------|----------------|
| Rust        | 1.85 (edition 2024) |
| Python      | 3.10 |
| `pip` package `mcp` | latest |

### Clone and build

```bash
git clone https://github.com/jimmychau1997/sovereign-omni-tool.git
cd sovereign-omni-tool
cargo build
```

### Run the tests

```bash
cargo test          # Rust unit tests
cargo clippy        # Lint
cargo fmt --check   # Format check
```

---

## Project Structure

```
sovereign-omni-tool/
├── src/
│   └── main.rs         — Rust dispatcher (CLI, MCP server, tool discovery)
├── tools/              — Python tool scripts (one file per tool)
├── sov_mcp.py          — FastMCP wrapper for LLM clients
├── arsenal_dump.json   — Auto-generated tool descriptions (optional)
├── categorize_arsenal.py — Helper to regenerate arsenal_dump.json
├── Cargo.toml
├── pyproject.toml
└── .github/
    └── workflows/
        └── ci.yml
```

---

## Development Workflow

1. Fork the repo and create a feature branch:
   ```bash
   git checkout -b feat/my-feature
   ```
2. Make your changes and add tests where appropriate.
3. Ensure everything passes:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```
4. Commit using [Conventional Commits](#commit-messages).
5. Open a Pull Request against `main`.

---

## Adding a New Tool

Each tool is a standalone Python script placed in the `tools/` directory.

### Requirements

- **File name**: `tools/<tool_name>.py` (snake_case).
- **Entry point**: the script must run when invoked directly:
  ```bash
  python3 tools/my_tool.py [args...]
  ```
- **`--help`**: accept a `--help` / `-h` flag and print a usage summary.
- **No side-effects on import** — wrap execution in `if __name__ == "__main__":`.
- **Standard-library only** where possible to keep the dependency footprint low.
  If third-party packages are needed, document them in `tools/README.md`.
- **Tests**: place unit tests in `tools/test_<tool_name>.py`; the dispatcher
  ignores files prefixed with `test_`.

### Regenerating `arsenal_dump.json`

After adding tools, refresh the metadata:

```bash
cargo build
python3 categorize_arsenal.py
```

---

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/):

```
<type>(<scope>): <short description>

[optional body]
[optional footer]
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`.

Examples:
```
feat(tools): add xml_to_json converter
fix(mcp): handle missing arsenal_dump.json gracefully
docs: update installation instructions
```

---

## Pull Request Guidelines

- Keep PRs focused on a single concern.
- Update `CHANGELOG.md` under an `[Unreleased]` section.
- All CI checks must pass before merging.
- Add or update tests when changing behaviour.
- Squash-merge is preferred for feature branches.

---

## Reporting Bugs

Open an issue and include:
- `sov --version` output.
- OS and Python version.
- The full command you ran.
- The full error output.

---

## Requesting Features

Open an issue with the label **enhancement** and describe:
- The problem you are trying to solve.
- The proposed solution or API.
- Alternatives you considered.
