import os
import re

base_dir = "/home/c/.gemini/antigravity/scratch/forge_v2/"

def patch(file_path, old_text, new_text):
    if not os.path.exists(file_path): return
    with open(file_path, "r") as f: content = f.read()
    if old_text in content:
        content = content.replace(old_text, new_text)
        with open(file_path, "w") as f: f.write(content)

# 2. changelog_generator.py
patch(base_dir + "output_glm5turbo/changelog_generator.py", "import sys", "import sys\nimport argparse")

# 3. file_merger.py
patch(base_dir + "output_glm5turbo/file_merger.py", 
      "print(f\"... ({len(content.split('\\n')) - lines} more lines)\")", 
      "num = len(content.split('\\n')) - lines\n            print(f\"... ({num} more lines)\")")

# 4. json_diff.py
patch(base_dir + "output_glm5turbo/json_diff.py",
      "f\"\\n  {self.colorize(f'- {diff[\"old_type\"]}: {old_str}', 'red')}\" +",
      "f\"\\n  {self.colorize('- ' + str(diff.get('old_type')) + ': ' + str(old_str), 'red')}\" +")

# 5. log_aggregator.py
patch(base_dir + "output_glm5turbo/log_aggregator.py",
      "add_parser.add_argument('--format', choices=['apache', 'nginx', 'syslog', 'json'], help='Log format')",
      "") # Remove duplicate if it was already added

# 6. password_gen.py
patch(base_dir + "output_glm5turbo/password_gen.py",
      "f\"{c(f'Strength: {info[\\\"level\\\"]}', info['color'])}\")",
      "str(c('Strength: ' + info['level'], info['color'])) )")

# 7. process_monitor.py
patch(base_dir + "output_glm5turbo/process_monitor.py",
      "(colorize(proc.status:<10, status_color))",
      "(colorize(f\"{proc.status:<10}\", status_color))")

# 8. secret_scanner.py
patch(base_dir + "output_glm5turbo/secret_scanner.py",
      "r'(?i)aws(.{0,20})?['\"][0-9a-zA-Z/+=]{40}['\"]'",
      "r'(?i)aws(.{0,20})?[\'\"][0-9a-zA-Z/+=]{40}[\'\"]'")

# 9. sql_builder.py
patch(base_dir + "output_glm45air/sql_builder.py",
      "insert_query, insert_params = builder.insert({",
      "builder.insert({")

# 10. sql_builder_clean.py
patch(base_dir + "output_glm45air/sql_builder_clean.py",
      "return f\"'{value.replace(\"'\", \"''\")}'\"",
      "s = value.replace(\"'\", \"''\")\n        return f\"'{s}'\"")

# 11. system_info_reporter.py
patch(base_dir + "output_glm5turbo/system_info_reporter.py", "import sys", "import sys\nimport argparse")

# 12. timezone_converter.py
patch(base_dir + "output_glm5turbo/timezone_converter.py",
      "f\"\\n{colorize(f'Search: \"{args.search}\"', Colors.YELLOW)}\"",
      "f\"\\n{colorize('Search: ' + str(args.search), Colors.YELLOW)}\"")

print("Patching complete!")
