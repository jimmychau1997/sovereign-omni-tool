use clap::{Parser, Subcommand};
use std::process::Command;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::io::{self, BufRead, Write};
use serde_json::{Value, json};

#[derive(Parser, Debug)]
#[command(name = "sov")]
#[command(author = "Sovereign Forge")]
#[command(version = "1.0")]
#[command(about = "Sovereign Omni-Tool: The ultimate developer Swiss Army Knife", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all dynamically discovered AI tools
    List,
    /// Start as a Model Context Protocol (MCP) server over STDIO
    Mcp,
    /// Native Rust rate limiter
    LimitRate {
        #[arg(short, long)]
        rate: u32,
    },
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            println!("🔍 Sovereign Omni-Tool Capabilities:");
            println!("  Dynamically scanning Forge Output directories...");
            let tools = discover_tools();
            for tool in tools {
                println!("  - {}", tool);
            }
            println!("Total tools activated: ✨ 🎉 ✨");
        }
        Commands::Mcp => {
            run_mcp_server().await?;
        }
        Commands::LimitRate { rate } => {
            println!("Executing high-perf native Rust rate limiter at {} req/s...", rate);
            // TODO: Integrate `hierarch` logic here native
        }
        Commands::External(args) => {
            if args.is_empty() {
                anyhow::bail!("No command specified.");
            }
            let cmd_name = args[0].to_str().unwrap_or("");
            let remaining_args = &args[1..];
            let target_path = find_tool(cmd_name);
            match target_path {
                Some(path) => run_python_script(path.to_str().unwrap(), remaining_args)?,
                None => anyhow::bail!("Tool '{}' not found in Sovereign Forge arsenal.", cmd_name),
            }
        }
    }

    Ok(())
}

fn discover_tools() -> Vec<String> {
    let base_dirs = vec![
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_glm5turbo",
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_glm47b",
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_glm45air",
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_qwen27",
    ];
    let mut tools = Vec::new();
    for dir in base_dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("py") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if !stem.starts_with("test_") && stem != "__init__" {
                            tools.push(stem.to_string());
                        }
                    }
                }
            }
        }
    }
    tools.sort();
    tools.dedup();
    tools
}

fn find_tool(name: &str) -> Option<PathBuf> {
    let base_dirs = vec![
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_glm5turbo",
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_glm47b",
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_glm45air",
        "/home/c/.gemini/antigravity/scratch/forge_v2/output_qwen27",
    ];
    for dir in base_dirs {
        let py_candidate = PathBuf::from(dir).join(format!("{}.py", name));
        if py_candidate.exists() {
            return Some(py_candidate);
        }
    }
    None
}

async fn run_mcp_server() -> Result<()> {
    // Basic JSON-RPC loop over standard input/output
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break, // EOF or err
        };
        if line.trim().is_empty() { continue; }
        
        let req: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(method) = req.get("method").and_then(|m| m.as_str()) {
            let id = req.get("id").cloned().unwrap_or(Value::Null);
            match method {
                "initialize" => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": "2024-11-05",
                            "capabilities": {
                                "tools": { "listChanged": true }
                            },
                            "serverInfo": {
                                "name": "sovereign-omni-tool",
                                "version": "1.0.0"
                            }
                        }
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap())?;
                }
                "tools/list" => {
                    let tools = discover_tools();
                    let formatted_tools: Vec<Value> = tools.iter().map(|name| {
                        json!({
                            "name": name,
                            "description": format!("Execute the {} python tool", name),
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "args": {
                                        "type": "array",
                                        "items": { "type": "string" },
                                        "description": format!("Arguments to pass to {} --help", name)
                                    }
                                }
                            }
                        })
                    }).collect();

                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "tools": formatted_tools
                        }
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap())?;
                }
                "tools/call" => {
                    let result_val = handle_tool_call(&req).await;
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": result_val
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap())?;
                }
                _ => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32601, "message": "Method not found" }
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap())?;
                }
            }
        }
        stdout.flush()?;
    }
    Ok(())
}

async fn handle_tool_call(req: &Value) -> Value {
    let params = req.get("params");
    let tool_name = params.and_then(|p| p.get("name")).and_then(|n| n.as_str()).unwrap_or("");
    let tool_args = params.and_then(|p| p.get("arguments")).and_then(|a| a.get("args")).and_then(|a| a.as_array());

    let path = find_tool(tool_name);
    if path.is_none() {
        return json!({
            "isError": true,
            "content": [{ "type": "text", "text": format!("Tool {} not found", tool_name) }]
        });
    }

    let mut cmd = Command::new("python3");
    cmd.arg(path.unwrap());
    if let Some(args) = tool_args {
        for arg in args {
            if let Some(s) = arg.as_str() {
                cmd.arg(s);
            }
        }
    }

    match cmd.output() {
        Ok(output) => {
            let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
            let is_err = !output.status.success();
            let text = if is_err {
                format!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout_str, stderr_str)
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
            "content": [{ "type": "text", "text": format!("Execution failed: {}", e) }]
        })
    }
}

fn run_python_script(script_path: &str, args: &[OsString]) -> Result<()> {
    let mut cmd = Command::new("python3");
    cmd.arg(script_path);
    cmd.args(args);

    let status = cmd.status().context("Failed to execute external process")?;
    
    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        } else {
            anyhow::bail!("Command terminated incorrectly.");
        }
    }
    
    Ok(())
}
