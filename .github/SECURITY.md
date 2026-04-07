# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.x     | ✅ Yes    |
| < 1.0   | ❌ No     |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

If you discover a security issue in Sovereign Omni-Tool, please report it
responsibly by opening a
[GitHub Security Advisory](https://github.com/jimmychau1997/sovereign-omni-tool/security/advisories/new)
(private, only visible to maintainers).

Include as much of the following as possible:

- A clear description of the vulnerability.
- Steps to reproduce or a proof-of-concept.
- The affected version(s) of `sov`.
- Any suggested mitigation or fix.

## Response Timeline

We aim to:

- **Acknowledge** your report within **72 hours**.
- **Provide an initial assessment** within **7 days**.
- **Release a patch** for confirmed vulnerabilities as promptly as possible.

We will credit reporters in the release notes unless you prefer to remain
anonymous.

## Scope

This policy covers the core `sov` binary (`src/main.rs`) and the MCP server
wrapper (`sov_mcp.py`). Individual tool scripts under `tools/` are
community-contributed; please report issues affecting them in the same way.

## Security Best Practices for Users

- Always build from a tagged release or a verified commit.
- Do not run `sov` as root unless strictly necessary.
- Review any community tool scripts before adding them to your `tools/` directory.
- Keep your Rust toolchain and Python interpreter up to date.
