use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;

// ─── CLI definition ──────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
#[command(name = "sov")]
#[command(author = "Sovereign Forge")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = "Sovereign Omni-Tool — the ultimate developer Swiss Army Knife",
    long_about = "\
Sovereign Omni-Tool (sov) is a unified developer toolkit that combines the\n\
execution speed of Rust with the extensive ecosystem of Python.\n\
\n\
Tools are discovered from:\n\
  1. $SOV_TOOLS_PATH (colon-separated list of directories)\n\
  2. <binary-dir>/tools/\n\
  3. <binary-dir>/../tools/\n\
  4. <binary-dir>/../share/sov/tools/\n\
  5. ~/.local/share/sov/tools/  (Unix) / %APPDATA%\\sov\\tools\\  (Windows)\n\
\n\
See https://github.com/jimmychau1997/sovereign-omni-tool for documentation."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all available tools
    List {
        /// Output the list as a JSON array
        #[arg(long)]
        json: bool,
    },

    /// Start as a Model Context Protocol (MCP) server over STDIO
    Mcp,

    /// Throttle stdin lines to a fixed rate (lines per second).
    ///
    /// Reads every line from stdin and re-emits it to stdout while ensuring
    /// the throughput does not exceed `--rate` lines per second.
    LimitRate {
        /// Maximum number of lines per second to forward
        #[arg(short, long)]
        rate: u32,
    },

    #[command(external_subcommand)]
    External(Vec<OsString>),
}

// ─── Entry point ─────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { json } => {
            let tools = discover_tools();
            if *json {
                println!("{}", serde_json::to_string_pretty(&tools).unwrap());
            } else {
                println!(
                    "🔍 Sovereign Omni-Tool — {} tool(s) available:\n",
                    tools.len()
                );
                if tools.is_empty() {
                    eprintln!("  ⚠  No tools found.");
                    eprintln!("  Place Python tool scripts in one of these directories:\n");
                    for dir in get_tool_dirs() {
                        eprintln!("    • {}", dir.display());
                    }
                    eprintln!("\n  Or set the SOV_TOOLS_PATH environment variable.");
                } else {
                    for tool in &tools {
                        println!("  • {tool}");
                    }
                    println!("\n  ✨ Total: {} tool(s)", tools.len());
                }
            }
        }

        Commands::Mcp => {
            run_mcp_server().await?;
        }

        Commands::LimitRate { rate } => {
            if *rate == 0 {
                anyhow::bail!("--rate must be greater than zero");
            }
            let delay = std::time::Duration::from_secs_f64(1.0 / f64::from(*rate));
            let stdin = io::stdin();
            let stdout = io::stdout();
            let mut out = stdout.lock();
            for line in stdin.lock().lines() {
                let line = line.context("Failed to read from stdin")?;
                writeln!(out, "{line}")?;
                out.flush()?;
                std::thread::sleep(delay);
            }
        }

        Commands::External(args) => {
            if args.is_empty() {
                anyhow::bail!("No command specified.");
            }
            let cmd_name = args[0].to_str().unwrap_or("");
            let remaining_args = &args[1..];
            match find_tool(cmd_name) {
                Some(path) => run_python_script(path.to_str().unwrap(), remaining_args)?,
                None => {
                    anyhow::bail!(
                        "Tool '{}' not found. Run `sov list` to see available tools.",
                        cmd_name
                    );
                }
            }
        }
    }

    Ok(())
}

// ─── Tool discovery ───────────────────────────────────────────────────────────

/// Returns an ordered list of directories to search for tool scripts.
///
/// Search order:
/// 1. `$SOV_TOOLS_PATH` (OS path-separator-delimited)
/// 2. `<binary_dir>/tools/`
/// 3. `<binary_dir>/../tools/`       (e.g. `target/release` → repo root)
/// 4. `<binary_dir>/../share/sov/tools/` (FHS install layout)
/// 5. `~/.local/share/sov/tools/`    (Unix user data dir)
///    `%APPDATA%\sov\tools\`          (Windows user data dir)
fn get_tool_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    // 1. Environment variable override
    if let Ok(raw) = std::env::var("SOV_TOOLS_PATH") {
        dirs.extend(std::env::split_paths(&raw));
    }

    // 2-4. Paths relative to the running binary
    if let Ok(exe) = std::env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        dirs.push(exe_dir.join("tools"));
        dirs.push(exe_dir.join("..").join("tools"));
        dirs.push(exe_dir.join("..").join("share").join("sov").join("tools"));
    }

    // 5. Platform-specific user data directory
    #[cfg(unix)]
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(
            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("sov")
                .join("tools"),
        );
    }
    #[cfg(windows)]
    if let Ok(appdata) = std::env::var("APPDATA") {
        dirs.push(PathBuf::from(appdata).join("sov").join("tools"));
    }

    dirs
}

/// Scans the given directories and returns a sorted, deduplicated list of tool names.
fn discover_tools_in_dirs(dirs: &[PathBuf]) -> Vec<String> {
    let mut tools = Vec::new();
    for dir in dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("py")
                    && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                    && !stem.starts_with("test_")
                    && stem != "__init__"
                {
                    tools.push(stem.to_string());
                }
            }
        }
    }
    tools.sort();
    tools.dedup();
    tools
}

/// Scans all tool directories and returns a sorted, deduplicated list of tool names.
fn discover_tools() -> Vec<String> {
    discover_tools_in_dirs(&get_tool_dirs())
}

/// Searches `dirs` for `<name>.py` and returns the first match.
fn find_tool_in_dirs(name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    for dir in dirs {
        let candidate = dir.join(format!("{name}.py"));
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

/// Searches all tool directories for `<name>.py` and returns the first match.
fn find_tool(name: &str) -> Option<PathBuf> {
    find_tool_in_dirs(name, &get_tool_dirs())
}

/// Loads optional tool descriptions from `arsenal_dump.json` located next to
/// the binary (or up to two parent directories above it).
fn load_arsenal_descriptions() -> HashMap<String, String> {
    let candidates: Vec<Option<PathBuf>> = {
        let exe_opt = std::env::current_exe().ok();
        vec![
            exe_opt
                .as_ref()
                .and_then(|e| e.parent())
                .map(|d| d.join("arsenal_dump.json")),
            exe_opt
                .as_ref()
                .and_then(|e| e.parent()?.parent())
                .map(|d| d.join("arsenal_dump.json")),
            exe_opt
                .as_ref()
                .and_then(|e| e.parent()?.parent()?.parent())
                .map(|d| d.join("arsenal_dump.json")),
        ]
    };

    for path in candidates.into_iter().flatten() {
        if let Ok(content) = std::fs::read_to_string(&path)
            && let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&content)
        {
            return map;
        }
    }
    HashMap::new()
}

// ─── MCP server ───────────────────────────────────────────────────────────────

async fn run_mcp_server() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }

        let req: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(method) = req.get("method").and_then(|m| m.as_str()) {
            let id = req.get("id").cloned().unwrap_or(Value::Null);
            let resp = match method {
                "initialize" => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": { "tools": { "listChanged": true } },
                        "serverInfo": {
                            "name": "sovereign-omni-tool",
                            "version": env!("CARGO_PKG_VERSION")
                        }
                    }
                }),

                "tools/list" => {
                    let descriptions = load_arsenal_descriptions();
                    let tools = discover_tools();
                    let formatted: Vec<Value> = tools
                        .iter()
                        .map(|name| {
                            let desc = descriptions
                                .get(name)
                                .filter(|s| !s.is_empty())
                                .cloned()
                                .unwrap_or_else(|| format!("Execute the {name} tool"));
                            json!({
                                "name": name,
                                "description": desc,
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "args": {
                                            "type": "array",
                                            "items": { "type": "string" },
                                            "description": "Command-line arguments to pass to the tool"
                                        }
                                    }
                                }
                            })
                        })
                        .collect();
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": { "tools": formatted }
                    })
                }

                "tools/call" => {
                    let result_val = handle_tool_call(&req).await;
                    json!({ "jsonrpc": "2.0", "id": id, "result": result_val })
                }

                _ => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": "Method not found" }
                }),
            };

            writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap())?;
            stdout.flush()?;
        }
    }
    Ok(())
}

async fn handle_tool_call(req: &Value) -> Value {
    let params = req.get("params");
    let tool_name = params
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("");
    let tool_args = params
        .and_then(|p| p.get("arguments"))
        .and_then(|a| a.get("args"))
        .and_then(|a| a.as_array());

    let Some(path) = find_tool(tool_name) else {
        return json!({
            "isError": true,
            "content": [{ "type": "text", "text": format!("Tool '{tool_name}' not found") }]
        });
    };

    let mut cmd = Command::new("python3");
    cmd.arg(&path);
    if let Some(args) = tool_args {
        for arg in args {
            if let Some(s) = arg.as_str() {
                cmd.arg(s);
            }
        }
    }

    match cmd.output() {
        Ok(output) => {
            let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();
            let is_err = !output.status.success();
            let text = if is_err {
                format!("STDOUT:\n{stdout_str}\n\nSTDERR:\n{stderr_str}")
            } else {
                stdout_str
            };
            json!({
                "isError": is_err,
                "content": [{ "type": "text", "text": text }]
            })
        }
        Err(e) => json!({
            "isError": true,
            "content": [{ "type": "text", "text": format!("Execution failed: {e}") }]
        }),
    }
}

// ─── Utilities ────────────────────────────────────────────────────────────────

fn run_python_script(script_path: &str, args: &[OsString]) -> Result<()> {
    let status = Command::new("python3")
        .arg(script_path)
        .args(args)
        .status()
        .context("Failed to launch Python interpreter")?;

    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        } else {
            anyhow::bail!("Python script terminated by signal");
        }
    }
    Ok(())
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_tool_dir(dir: &std::path::Path, names: &[&str]) {
        for name in names {
            fs::write(dir.join(format!("{name}.py")), b"# tool").unwrap();
        }
    }

    #[test]
    fn discover_filters_test_and_init_files() {
        let tmp = TempDir::new().unwrap();
        make_tool_dir(tmp.path(), &["foo", "bar", "test_ignored", "__init__"]);

        let tools = discover_tools_in_dirs(&[tmp.path().to_path_buf()]);

        assert!(
            tools.contains(&"foo".to_string()),
            "expected 'foo' in {tools:?}"
        );
        assert!(
            tools.contains(&"bar".to_string()),
            "expected 'bar' in {tools:?}"
        );
        assert!(
            !tools.contains(&"test_ignored".to_string()),
            "test_ prefix should be excluded"
        );
        assert!(
            !tools.contains(&"__init__".to_string()),
            "__init__ should be excluded"
        );
    }

    #[test]
    fn find_tool_locates_py_file() {
        let tmp = TempDir::new().unwrap();
        make_tool_dir(tmp.path(), &["my_tool"]);

        let result = find_tool_in_dirs("my_tool", &[tmp.path().to_path_buf()]);

        assert!(result.is_some(), "should find my_tool.py");
        assert!(result.unwrap().exists());
    }

    #[test]
    fn find_tool_returns_none_for_missing_tool() {
        let tmp = TempDir::new().unwrap();
        let result = find_tool_in_dirs(
            "definitely_not_a_real_tool_xyz",
            &[tmp.path().to_path_buf()],
        );
        assert!(result.is_none());
    }

    #[test]
    fn tools_list_is_sorted_and_deduplicated() {
        let tmp1 = TempDir::new().unwrap();
        let tmp2 = TempDir::new().unwrap();
        make_tool_dir(tmp1.path(), &["zebra", "apple"]);
        make_tool_dir(tmp2.path(), &["apple", "mango"]); // 'apple' appears in both dirs

        let tools = discover_tools_in_dirs(&[tmp1.path().to_path_buf(), tmp2.path().to_path_buf()]);

        assert_eq!(tools, vec!["apple", "mango", "zebra"]);
    }
}
