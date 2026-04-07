# Sovereign Omni-Tool (`sov`)

[![CI](https://github.com/jimmychau1997/sovereign-omni-tool/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/jimmychau1997/sovereign-omni-tool/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.10%2B-blue.svg)](https://www.python.org)

> A unified developer Swiss Army Knife — 114+ utilities spanning system ops,
> security recon, developer tooling, and data processing. Built with a
> blazing-fast Rust dispatcher and a pluggable Python tool ecosystem.

---

## Table of Contents

- [Features](#-features)
- [Tool Categories](#-tool-categories)
- [Installation](#-installation)
- [Usage](#-usage)
- [Tool Discovery](#-tool-discovery)
- [MCP Server](#-mcp-server)
- [Adding Your Own Tools](#-adding-your-own-tools)
- [Configuration](#-configuration)
- [Development](#-development)
- [Contributing](#-contributing)
- [License](#-license)

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| **Blazing-fast dispatcher** | Rust + Tokio — zero-latency tool invocation |
| **114+ utilities** | Spanning 4 functional domains |
| **Plug-in architecture** | Drop a `.py` file in `tools/` — it's immediately available |
| **MCP Server ready** | Expose every tool to any LLM client in one command |
| **Portable** | No hardcoded paths — works anywhere out of the box |
| **Dependency-light** | Tools rely on the Python standard library wherever possible |

---

## 📦 Tool Categories

### 1 — System & DevOps Operations

`ansible_runner` · `backup_manager` · `cicd_pipeline_generator` ·
`cron_manager` · `database_backup` · `disk_space_analyzer` ·
`docker_compose_generator` · `env_var_manager` · `environment_manager` ·
`file_watcher` · `memory_profiler` · `process_monitor` · `service_manager` ·
`symlink_manager` · `system_benchmark` · `system_info_reporter` ·
`system_resource_monitor` · `terraform_validator`

### 2 — Security & Reconnaissance

`api_rate_limiter` · `certificate_parser` · `dns_lookup` ·
`http_client_tester` · `http_logger` · `http_request_builder` · `ip_calc` ·
`ip_geolocator` · `jwt_decoder` · `network_speed_tester` ·
`network_traffic_analyzer` · `password_gen` · `password_generator` ·
`port_scanner` · `rbac_policy_manager` · `secret_scanner` · `ssl_cert_checker`

### 3 — Developer Utilities

`api_doc_generator` · `api_mock_server` · `assertion_lib` ·
`changelog_generator` · `code_counter` · `code_line_counter` ·
`dependency_checker` · `dependency_scanner` · `git_statistics` ·
`license_generator` · `mock_generator` · `sql_builder` · `sql_builder_clean` ·
`sql_query_builder` · `task_timer` · `todo_scanner` · `websocket_client`

### 4 — Data & Media Processing

`base64_codec` · `batch_processor` · `checksum_calculator` · `clipboard_manager` ·
`color_converter` · `color_themes` · `config_validator` · `csv_analyzer` ·
`csv_merger` · `csv_to_json` · `data_serializer` · `data_validator` ·
`duplicate_file_finder` · `email_validator` · `encoding_detector` ·
`excel_to_csv` · `file_merger` · `file_renamer` · `file_splitter` ·
`hash_calculator` · `hash_gen` · `image_metadata_extractor` · `image_resizer` ·
`image_watermarker` · `json_diff` · `json_pretty` · `json_schema_validator` ·
`json_to_sql` · `json_yaml_converter` · `log_aggregator` · `log_file_analyzer` ·
`log_parser` · `lorem_gen` · `lorem_generator` · `markdown_formatter` ·
`markdown_table_generator` · `markdown_to_html` · `mock_generator` ·
`pdf_toolkit` · `qr_generator` · `random_data_generator` · `regex_tester` ·
`report_generator` · `slug_gen` · `slug_generator` · `text_diff` ·
`text_encoding_detector` · `text_replacer` · `timestamp_converter` ·
`timezone_converter` · `unit_converter` · `url_encoder` · `uuid_gen` ·
`xml_formatter` · `xml_to_json` · `yaml_validator` _(and more)_

---

## 🚀 Installation

### Prerequisites

- [Rust](https://rustup.rs) ≥ 1.85
- Python ≥ 3.10

### Build from source

```bash
git clone https://github.com/jimmychau1997/sovereign-omni-tool.git
cd sovereign-omni-tool
cargo build --release
```

The binary is at `target/release/sov`.

### Add to PATH

```bash
# Option A — copy to /usr/local/bin
sudo cp target/release/sov /usr/local/bin/

# Option B — symlink
ln -s "$(pwd)/target/release/sov" ~/.local/bin/sov

# Option C — cargo install (once published to crates.io)
cargo install sov
```

### Install Python tools

Place your tool scripts in the `tools/` directory (see
[Tool Discovery](#-tool-discovery)) and verify:

```bash
sov list
```

---

## 🛠 Usage

```
sov [COMMAND]
```

| Command | Description |
|---------|-------------|
| `sov list` | List all discovered tools |
| `sov list --json` | Machine-readable JSON list of tools |
| `sov <tool> [args...]` | Run a specific tool |
| `sov mcp` | Start the MCP server over STDIO |
| `sov limit-rate --rate <N>` | Throttle stdin to N lines/second |
| `sov --help` | Full help |
| `sov --version` | Print version |

### Examples

```bash
# List all available tools
sov list

# Run a tool
sov port_scanner --range 1-1024 example.com
sov csv_to_json data.csv
sov password_gen --strength strong
sov dns_lookup --all github.com
sov hash_calculator myfile.zip
sov base64_codec encode "hello world"

# Get help for any tool
sov <tool_name> --help
```

---

## 🔍 Tool Discovery

`sov` searches for `*.py` files in these directories (in order):

| Priority | Location |
|----------|----------|
| 1 | `$SOV_TOOLS_PATH` — colon-separated list (`:` on Unix, `;` on Windows) |
| 2 | `<binary-dir>/tools/` |
| 3 | `<binary-dir>/../tools/` |
| 4 | `<binary-dir>/../share/sov/tools/` (FHS install layout) |
| 5 | `~/.local/share/sov/tools/` (Unix) / `%APPDATA%\sov\tools\` (Windows) |

Files whose names start with `test_` or equal `__init__` are excluded.

### Override example

```bash
export SOV_TOOLS_PATH="/opt/my-tools:/home/alice/tools"
sov list
```

---

## 🤖 MCP Server

`sov` ships with a [FastMCP](https://github.com/jlowin/fastmcp) wrapper that
exposes all tools to any MCP-compatible LLM client (e.g. Claude Desktop).

### Start the server

```bash
pip install mcp
python3 sov_mcp.py
```

### Configuration

| Environment variable | Default | Description |
|----------------------|---------|-------------|
| `SOV_BIN` | `sov` on `$PATH`, then build outputs | Path to the `sov` binary |
| `SOV_DUMP_PATH` | `arsenal_dump.json` next to the script | Path to the tool-descriptions file |

### Claude Desktop integration

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "sovereign-omni-tool": {
      "command": "python3",
      "args": ["/path/to/sovereign-omni-tool/sov_mcp.py"]
    }
  }
}
```

---

## 🔧 Adding Your Own Tools

1. Create `tools/<your_tool>.py` — see [`tools/README.md`](tools/README.md) for
   the template and requirements.
2. Verify it appears in the tool list:
   ```bash
   sov list
   sov your_tool --help
   ```
3. _(Optional)_ Regenerate the description cache:
   ```bash
   python3 categorize_arsenal.py
   ```

---

## ⚙️ Configuration

| Variable | Description |
|----------|-------------|
| `SOV_TOOLS_PATH` | Colon-separated list of directories to search for tool scripts |
| `SOV_BIN` | Path to `sov` binary used by `sov_mcp.py` |
| `SOV_DUMP_PATH` | Path to `arsenal_dump.json` used by `sov_mcp.py` |

---

## 💻 Development

```bash
# Build
cargo build

# Run all Rust tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Python lint
pip install ruff
ruff check sov_mcp.py
```

---

## 🤝 Contributing

Contributions are very welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md)
before opening a pull request.

We follow the [Contributor Covenant](CODE_OF_CONDUCT.md) code of conduct.

---

## 📄 License

[MIT](LICENSE) © 2025 Sovereign Forge

