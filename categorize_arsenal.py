import sys
import subprocess
import json
import re

tools = [
"ansible_runner", "api_doc_generator", "api_mock_server", "api_rate_limiter", "assertion_lib", "backup_manager", "base64_codec", "batch_processor", "certificate_parser", "changelog_generator", "checksum_calculator", "cicd_pipeline_generator", "clipboard_manager", "code_counter", "code_line_counter", "color_converter", "color_themes", "config_validator", "cron_manager", "csv_analyzer", "csv_merger", "csv_to_json", "data_serializer", "data_validator", "database_backup", "dependency_checker", "dependency_scanner", "disk_space_analyzer", "dns_lookup", "docker_compose_generator", "duplicate_file_finder", "email_validator", "encoding_detector", "env_var_manager", "environment_manager", "excel_to_csv", "file_merger", "file_renamer", "file_splitter", "file_watcher", "git_statistics", "hash_calculator", "hash_gen", "http_client_tester", "http_logger", "http_request_builder", "image_metadata_extractor", "image_resizer", "image_watermarker", "ip_calc", "ip_geolocator", "json_diff", "json_pretty", "json_schema_validator", "json_to_sql", "json_yaml_converter", "jwt_decoder", "license_generator", "log_aggregator", "log_file_analyzer", "log_parser", "lorem_gen", "lorem_generator", "markdown_formatter", "markdown_table_generator", "markdown_to_html", "memory_profiler", "mock_generator", "network_speed_tester", "network_traffic_analyzer", "password_gen", "password_generator", "pdf_toolkit", "port_scanner", "process_monitor", "qr_generator", "random_data_generator", "rbac_policy_manager", "regex_tester", "report_generator", "secret_scanner", "service_manager", "slug_gen", "slug_generator", "sql_builder", "sql_builder_clean", "sql_query_builder", "ssl_cert_checker", "symlink_manager", "system_benchmark", "system_info_reporter", "system_resource_monitor", "task_timer", "terraform_validator", "text_diff", "text_encoding_detector", "text_replacer", "timestamp_converter", "timezone_converter", "todo_scanner", "unit_converter", "url_encoder", "uuid_gen", "websocket_client", "xml_formatter", "xml_to_json", "yaml_validator"
]

results = {}

for tool in tools:
    try:
        out = subprocess.check_output(["./target/debug/sov", tool, "--help"], stderr=subprocess.STDOUT, text=True)
        lines = out.split('\n')
        desc = ""
        for line in lines:
            line_l = line.lower()
            if "usage:" in line_l or "positional arguments:" in line_l or "options:" in line_l or "optional arguments:" in line_l:
                continue
            if line.strip() and not line.strip().startswith("-") and "Sovereign Omni-Tool" not in line:
                desc += line.strip() + " "
                if len(desc) > 150:
                    break
        results[tool] = desc.strip()[:150]
    except Exception as e:
        results[tool] = "Error executing help"

with open("arsenal_dump.json", "w") as f:
    json.dump(results, f, indent=2)

print("Dump complete")
