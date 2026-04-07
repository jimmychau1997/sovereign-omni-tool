## Description

<!-- Briefly describe what this PR does and why it is needed. -->

Closes #<!-- issue number, or remove this line if not applicable -->

## Type of Change

- [ ] 🐛 Bug fix (non-breaking change that fixes an issue)
- [ ] ✨ New feature (non-breaking change that adds functionality)
- [ ] 🔨 New tool added to `tools/`
- [ ] 🔧 Refactor (no behaviour change)
- [ ] 📝 Documentation update
- [ ] ⚙️  CI / build / tooling change
- [ ] 💥 Breaking change (fix or feature that would break existing behaviour)

## How Has This Been Tested?

<!-- Describe the tests you ran and the commands used. -->

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
# If Python files changed:
ruff check sov_mcp.py categorize_arsenal.py
```

## Checklist

- [ ] I have read [CONTRIBUTING.md](../CONTRIBUTING.md)
- [ ] My branch is up to date with `master`
- [ ] All CI checks pass locally
- [ ] I have added or updated tests where appropriate
- [ ] I have updated `CHANGELOG.md` under `[Unreleased]`
- [ ] Documentation has been updated if behaviour changed
- [ ] If a new tool was added: `arsenal_dump.json` has been regenerated (`python3 categorize_arsenal.py`)
