"""
Sovereign Omni-Tool — FastMCP server wrapper.

This module exposes all sov tools as MCP tool endpoints so that any
MCP-compatible LLM client (e.g. Claude Desktop, OpenClaw) can invoke them.

Configuration (environment variables):
  SOV_BIN        — path to the `sov` binary   (default: `sov` on $PATH, then
                   the release/debug build next to this file)
  SOV_DUMP_PATH  — path to arsenal_dump.json  (default: next to this file)
"""

from __future__ import annotations

import json
import os
import shlex
import shutil
import subprocess
from pathlib import Path

from mcp.server.fastmcp import FastMCP

# ── Binary discovery ──────────────────────────────────────────────────────────

_SCRIPT_DIR = Path(__file__).resolve().parent


def _find_sov_bin() -> str:
    """Return the path to the `sov` binary, searching in order:
    1. SOV_BIN env var
    2. `sov` on $PATH
    3. <script_dir>/target/release/sov
    4. <script_dir>/target/debug/sov
    """
    if env := os.environ.get("SOV_BIN"):
        return env
    if which := shutil.which("sov"):
        return which
    for candidate in (
        _SCRIPT_DIR / "target" / "release" / "sov",
        _SCRIPT_DIR / "target" / "debug" / "sov",
    ):
        if candidate.exists():
            return str(candidate)
    return "sov"  # final fallback — let subprocess raise a clear error


def _find_dump_path() -> Path:
    """Return the path to arsenal_dump.json."""
    if env := os.environ.get("SOV_DUMP_PATH"):
        return Path(env)
    return _SCRIPT_DIR / "arsenal_dump.json"


SOV_BIN = _find_sov_bin()
DUMP_PATH = _find_dump_path()

# ── Arsenal metadata ──────────────────────────────────────────────────────────

try:
    with open(DUMP_PATH) as f:
        arsenal: dict[str, str] = json.load(f)
except Exception:
    arsenal = {}

# ── Tool categories ───────────────────────────────────────────────────────────

SYS_TOOLS = [
    "ansible_runner", "backup_manager", "cron_manager", "database_backup",
    "disk_space_analyzer", "docker_compose_generator", "env_var_manager",
    "environment_manager", "file_watcher", "memory_profiler", "process_monitor",
    "service_manager", "symlink_manager", "system_benchmark",
    "system_info_reporter", "system_resource_monitor", "terraform_validator",
    "cicd_pipeline_generator",
]
SEC_TOOLS = [
    "api_rate_limiter", "certificate_parser", "dns_lookup", "http_client_tester",
    "http_logger", "http_request_builder", "ip_calc", "ip_geolocator",
    "jwt_decoder", "network_speed_tester", "password_gen", "password_generator",
    "port_scanner", "rbac_policy_manager", "secret_scanner", "ssl_cert_checker",
    "network_traffic_analyzer",
]
DEV_TOOLS = [
    "api_doc_generator", "api_mock_server", "changelog_generator", "code_counter",
    "code_line_counter", "dependency_checker", "dependency_scanner",
    "git_statistics", "license_generator", "mock_generator", "sql_builder",
    "sql_builder_clean", "sql_query_builder", "todo_scanner", "websocket_client",
    "assertion_lib", "task_timer",
]
DATA_TOOLS = [t for t in arsenal if t not in SYS_TOOLS + SEC_TOOLS + DEV_TOOLS]

# ── MCP server ────────────────────────────────────────────────────────────────

mcp = FastMCP("Sovereign Omni-Tool Arsenal")


def _run_sov(tool_name: str, arguments: str) -> str:
    """Invoke `sov <tool_name> [arguments]` and return combined output."""
    if arsenal and tool_name not in arsenal:
        available = ", ".join(sorted(arsenal))
        return f"Error: Unknown tool '{tool_name}'. Available tools: {available}"

    cmd = [SOV_BIN, tool_name]
    if arguments:
        try:
            cmd.extend(shlex.split(arguments))
        except ValueError as exc:
            return f"Error parsing arguments: {exc}"

    try:
        result = subprocess.run(
            cmd, capture_output=True, text=True, timeout=120
        )
        out = result.stdout or ""
        if result.stderr:
            out += "\n[STDERR]\n" + result.stderr
        if result.returncode != 0:
            out = f"Command failed (exit {result.returncode}).\n" + out
        return out
    except subprocess.TimeoutExpired:
        return "Error: Command timed out after 120 seconds."
    except FileNotFoundError:
        return (
            f"Error: sov binary not found at '{SOV_BIN}'. "
            "Build with `cargo build --release` and ensure it is on $PATH, "
            "or set the SOV_BIN environment variable."
        )
    except Exception as exc:  # noqa: BLE001
        return f"Execution error: {exc}"


def _build_docstring(category_name: str, tool_list: list[str]) -> str:
    lines = [f"Execute an Omni-Tool from the **{category_name}** category.\n"]
    lines.append("Valid `tool_name` values and their capabilities:")
    for t in tool_list:
        desc = arsenal.get(t, "")
        lines.append(f"  - {t}: {desc}" if desc else f"  - {t}")
    lines.append(
        "\nPass `arguments` as a raw command-line string "
        "(e.g. `'--timeout 30 example.com'` or `'-h'`). "
        "Leave empty for no arguments."
    )
    return "\n".join(lines)


# ── Exposed MCP tools ─────────────────────────────────────────────────────────

@mcp.tool()
def sov_sys_ops(tool_name: str, arguments: str = "") -> str:  # noqa: D401
    """System & DevOps operations."""
    return _run_sov(tool_name, arguments)


sov_sys_ops.__doc__ = _build_docstring("System & DevOps", SYS_TOOLS)


@mcp.tool()
def sov_sec_recon(tool_name: str, arguments: str = "") -> str:  # noqa: D401
    """Security & Network reconnaissance."""
    return _run_sov(tool_name, arguments)


sov_sec_recon.__doc__ = _build_docstring("Security & Network", SEC_TOOLS)


@mcp.tool()
def sov_dev_utils(tool_name: str, arguments: str = "") -> str:  # noqa: D401
    """Developer utilities."""
    return _run_sov(tool_name, arguments)


sov_dev_utils.__doc__ = _build_docstring("Developer Utilities", DEV_TOOLS)


@mcp.tool()
def sov_data_media(tool_name: str, arguments: str = "") -> str:  # noqa: D401
    """Data & Media processing."""
    return _run_sov(tool_name, arguments)


sov_data_media.__doc__ = _build_docstring("Data & Media", DATA_TOOLS)


if __name__ == "__main__":
    mcp.run()


def main() -> None:
    """Entry point for the `sov-mcp` console script."""
    mcp.run()

