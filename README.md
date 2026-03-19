# Sovereign Omni-Tool (`sov`)

A unified developer toolkit built combining the execution speed of Rust with the extensive ecosystem of Python. 

Sovereign provides a single executable spanning over 114 different sub-commands, neatly packed into 4 core functional domains. It aims to provide convenience, speed, and standardisation to everyday development, security reconnaissance, and data processing tasks.

## 🛠 Features

- **Blazing Fast Dispatcher:** Built in Rust using Tokio, the `sov` dispatcher securely and instantly spawns Python subsystems with zero latency.
- **MCP Server Ready:** Comes out-of-the-box with a Python `FastMCP` wrapper (`sov_mcp.py`). Expose all 114 utilities instantly to any LLM (e.g. Claude or OpenClaw) without writing custom schemas.
- **Dependency Free Approach:** Most tools are completely native to standard Python libraries where possible.

## 📦 What's Inside? 

The 114 utilities are roughly categorized into four groups:

1. **System & DevOps Operations** 
   *Examples:* `ansible_runner`, `terraform_validator`, `docker_compose_generator`, `system_resource_monitor`.
2. **Security & Reconnaissance**
   *Examples:* `port_scanner`, `ip_geolocator`, `rbac_policy_manager`, `password_gen`, `secret_scanner`.
3. **Developer Tooling**
   *Examples:* `api_mock_server`, `sql_builder`, `code_line_counter`.
4. **Data & Media Processing**
   *Examples:* `csv_to_json`, `json_diff`, `image_resizer`, `hash_calculator`, `base64_codec`.

## 🚀 Getting Started

1. Clone the repository and build the Rust dispatcher:
   ```bash
   cargo build --release
   ```
2. The binary will be available at `target/release/sov`. 
3. Link or alias it to your `$PATH` for global use!
   ```bash
   sov list
   ```

### Using as an MCP Server

Start the bundled MCP API service directly:
```bash
pip install mcp
python3 sov_mcp.py
```

## 🤝 Contributing

We are extremely new and still actively building the ecosystem. All PRs, suggestions, and feature requests are warmly welcomed! 

---
*Built with ❤️ for the Open Source Community.*
