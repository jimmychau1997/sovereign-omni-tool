from mcp.server.fastmcp import FastMCP
import subprocess
import json
import shlex
import os

# Initialize FastMCP Server
mcp = FastMCP("Sovereign Omni-Tool Arsenal")

SOV_BIN = "/home/c/Desktop/Claw_Trinity/sov/target/debug/sov"
DUMP_PATH = "/home/c/Desktop/Claw_Trinity/sov/arsenal_dump.json"

try:
    with open(DUMP_PATH, "r") as f:
        arsenal = json.load(f)
except Exception:
    arsenal = {}

SYS_TOOLS = ["ansible_runner", "backup_manager", "cron_manager", "database_backup", "disk_space_analyzer", "docker_compose_generator", "env_var_manager", "environment_manager", "file_watcher", "memory_profiler", "process_monitor", "service_manager", "symlink_manager", "system_benchmark", "system_info_reporter", "system_resource_monitor", "terraform_validator", "cicd_pipeline_generator"]
SEC_TOOLS = ["api_rate_limiter", "certificate_parser", "dns_lookup", "http_client_tester", "http_logger", "http_request_builder", "ip_calc", "ip_geolocator", "jwt_decoder", "network_speed_tester", "password_gen", "password_generator", "port_scanner", "rbac_policy_manager", "secret_scanner", "ssl_cert_checker", "network_traffic_analyzer"]
DEV_TOOLS = ["api_doc_generator", "api_mock_server", "changelog_generator", "code_counter", "code_line_counter", "dependency_checker", "dependency_scanner", "git_statistics", "license_generator", "mock_generator", "sql_builder", "sql_builder_clean", "sql_query_builder", "todo_scanner", "websocket_client", "assertion_lib", "task_timer"]
DATA_TOOLS = [t for t in arsenal.keys() if t not in SYS_TOOLS + SEC_TOOLS + DEV_TOOLS]

def _run_sov(tool_name: str, arguments: str) -> str:
    if tool_name not in arsenal:
        return f"Error: Unknown tool '{tool_name}'. Available tools: {', '.join(list(arsenal.keys()))}"
    
    cmd = [SOV_BIN, tool_name]
    if arguments:
        try:
            cmd.extend(shlex.split(arguments))
        except ValueError as e:
            return f"Error parsing arguments: {e}"
            
    try:
        # Enforce timeout for safety
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
        out = ""
        if result.stdout: out += result.stdout
        if result.stderr: out += "\n[STDERR]\n" + result.stderr
        if result.returncode != 0:
            out = f"Command failed with exit code {result.returncode}.\n" + out
        return out
    except subprocess.TimeoutExpired:
        return "Error: Command timed out after 120 seconds."
    except Exception as e:
        return f"Execution error: {str(e)}"

def _build_docstring(category_name: str, tool_list: list) -> str:
    lines = [f"Execute an Omni-Tool from the {category_name} category."]
    lines.append("Valid `tool_name` values and their specific capabilities:")
    for t in tool_list:
        desc = arsenal.get(t, "")
        lines.append(f" - {t}: {desc}")
    lines.append("\nProvide `arguments` as a raw command-line string (e.g. '--timeout 30 my_domain.com' or '-h'). Leave empty for no arguments.")
    return "\n".join(lines)


@mcp.tool()
def sov_sys_ops(tool_name: str, arguments: str = "") -> str:
    return _run_sov(tool_name, arguments)
sov_sys_ops.__doc__ = _build_docstring("System & DevOps", SYS_TOOLS)

@mcp.tool()
def sov_sec_recon(tool_name: str, arguments: str = "") -> str:
    return _run_sov(tool_name, arguments)
sov_sec_recon.__doc__ = _build_docstring("Security & Network", SEC_TOOLS)

@mcp.tool()
def sov_dev_utils(tool_name: str, arguments: str = "") -> str:
    return _run_sov(tool_name, arguments)
sov_dev_utils.__doc__ = _build_docstring("Developer Utilities", DEV_TOOLS)

@mcp.tool()
def sov_data_media(tool_name: str, arguments: str = "") -> str:
    return _run_sov(tool_name, arguments)
sov_data_media.__doc__ = _build_docstring("Data & Media", DATA_TOOLS)

if __name__ == "__main__":
    mcp.run()
