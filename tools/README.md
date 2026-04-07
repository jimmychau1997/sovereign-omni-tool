# Tool Scripts Directory

This directory is the default location where `sov` looks for Python tool
scripts when the `SOV_TOOLS_PATH` environment variable is not set.

## Naming Convention

Each file must be named `<tool_name>.py` (snake_case) and be executable as:

```bash
python3 tools/<tool_name>.py [args...]
```

## Requirements for a Valid Tool Script

1. **`--help` / `-h` support** — the script must print usage information when
   invoked with `--help`.
2. **`if __name__ == "__main__":` guard** — ensure the entry point is protected
   so the file can also be imported without side effects.
3. **Standard library preferred** — avoid third-party dependencies unless
   absolutely necessary.
4. **No `test_` prefix** — files starting with `test_` are treated as test
   modules and are intentionally excluded from the tool list.

## Example Minimal Tool

```python
#!/usr/bin/env python3
"""My Tool — does something useful."""
import argparse


def main() -> None:
    parser = argparse.ArgumentParser(description="My Tool — does something useful.")
    parser.add_argument("input", help="Input value")
    args = parser.parse_args()
    print(f"Processing: {args.input}")


if __name__ == "__main__":
    main()
```

Save it as `tools/my_tool.py`, then run:

```bash
sov my_tool "hello world"
```

## Adding Tools to the Global Tools Path

You can also store tools in any directory and point `sov` at it:

```bash
export SOV_TOOLS_PATH="/path/to/my/tools:/another/path"
sov list
```

## Regenerating `arsenal_dump.json`

After adding new tools, refresh the metadata cache:

```bash
cargo build
python3 categorize_arsenal.py
```
