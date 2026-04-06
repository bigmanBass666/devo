use std::io::{self, BufRead, Write};
use std::sync::Arc;

use anyhow::Result;
use clap::Args;
use clawcr_core::{Message, QueryEvent, SessionConfig, SessionState, query};
use clawcr_safety::legacy_permissions::PermissionMode;
use clawcr_tools::{ToolOrchestrator, ToolRegistry};

use crate::config;

/// Output format for non-interactive (print/query) mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Plain text — assistant text only, streamed to stdout.
    Text,
    /// Newline-delimited JSON events (one JSON object per line).
    StreamJson,
    /// Single JSON object written after the turn completes.
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(OutputFormat::Text),
            "stream-json" => Ok(OutputFormat::StreamJson),
            "json" => Ok(OutputFormat::Json),
            other => anyhow::bail!("unknown output format '{}' (text|stream-json|json)", other),
        }
    }
}

/// Common agent-facing flags accepted by the main `clawcr` command.
#[derive(Debug, Args)]
pub struct AgentCli {
    /// Model to use (e.g. claude-sonnet-4-20250514, qwen3.5:9b)
    #[arg(short, long)]
    pub model: Option<String>,

    /// System prompt
    #[arg(
        short,
        long,
        default_value = "You are a helpful coding assistant. \
        Use tools when appropriate to help the user. Be concise."
    )]
    pub system: String,

    /// Permission mode: auto, interactive, deny
    #[arg(short, long, default_value = "auto")]
    pub permission: String,

    /// Run a single prompt non-interactively then exit
    #[arg(short = 'q', long)]
    pub query: Option<String>,

    /// Run a single prompt non-interactively then exit (alias for --query)
    #[arg(long)]
    pub print: Option<String>,

    /// Output format for non-interactive mode: text (default), stream-json, json
    #[arg(long, default_value = "text")]
    pub output_format: OutputFormat,

    /// Maximum turns per conversation
    #[arg(long, default_value = "100")]
    pub max_turns: usize,

    /// Provider: anthropic, ollama, openai (auto-detected if not set)
    #[arg(long)]
    pub provider: Option<String>,

    /// Ollama server URL
    #[arg(long, default_value = "http://localhost:11434")]
    pub ollama_url: String,
}

/// Runs the interactive or one-shot coding-agent entrypoint.
pub async fn run_agent(cli: AgentCli) -> Result<()> {
    let cwd = std::env::current_dir()?;

    let single_prompt = cli.query.or(cli.print);
    let interactive = single_prompt.is_none();

    let permission_mode = match cli.permission.as_str() {
        "auto" => PermissionMode::AutoApprove,
        "interactive" => PermissionMode::Interactive,
        "deny" => PermissionMode::Deny,
        other => {
            eprintln!("unknown permission mode '{}', using auto", other);
            PermissionMode::AutoApprove
        }
    };

    let mut registry = ToolRegistry::new();
    clawcr_tools::register_builtin_tools(&mut registry);
    let registry = Arc::new(registry);
    let orchestrator = ToolOrchestrator::new(Arc::clone(&registry));

    if cli.provider.as_deref() == Some("ollama") {
        config::ensure_ollama(&cli.ollama_url, interactive)?;
    }

    let resolved = config::resolve_provider(
        cli.provider.as_deref(),
        cli.model.as_deref(),
        &cli.ollama_url,
        interactive,
    )?;

    let session_config = SessionConfig {
        model: resolved.model,
        system_prompt: cli.system.clone(),
        max_turns: cli.max_turns,
        permission_mode,
        ..Default::default()
    };

    let mut session = SessionState::new(session_config, cwd);

    if let Some(prompt) = single_prompt {
        session.push_message(Message::user(prompt));
        let on_event = make_event_callback(cli.output_format);
        query(
            &mut session,
            resolved.provider.as_ref(),
            Arc::clone(&registry),
            &orchestrator,
            Some(on_event),
        )
        .await?;

        if cli.output_format == OutputFormat::Json {
            let last_assistant = session
                .messages
                .iter()
                .rev()
                .find(|message| matches!(message.role, clawcr_core::Role::Assistant));
            if let Some(message) = last_assistant {
                let text: String = message
                    .content
                    .iter()
                    .filter_map(|block| match block {
                        clawcr_core::ContentBlock::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                println!(
                    "{}",
                    serde_json::json!({
                        "type": "result",
                        "text": text,
                        "session_id": session.id,
                        "input_tokens": session.total_input_tokens,
                        "output_tokens": session.total_output_tokens,
                    })
                );
            }
        }

        return Ok(());
    }

    println!("ClawCR v{}", env!("CARGO_PKG_VERSION"));
    println!("Type your message, or 'exit' / Ctrl-D to quit.\n");

    let on_event = make_event_callback(OutputFormat::Text);
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "exit" || line == "quit" {
            break;
        }

        session.push_message(Message::user(line));

        if let Err(error) = query(
            &mut session,
            resolved.provider.as_ref(),
            Arc::clone(&registry),
            &orchestrator,
            Some(Arc::clone(&on_event)),
        )
        .await
        {
            eprintln!("error: {}", error);
        }
    }

    eprintln!(
        "\n[session: {} turns, {} in / {} out tokens]",
        session.turn_count, session.total_input_tokens, session.total_output_tokens
    );

    Ok(())
}

fn make_event_callback(format: OutputFormat) -> Arc<dyn Fn(QueryEvent) + Send + Sync> {
    Arc::new(move |event| match format {
        OutputFormat::Text => handle_event_text(event),
        OutputFormat::StreamJson => handle_event_stream_json(event),
        OutputFormat::Json => match &event {
            QueryEvent::ToolUseStart { name, .. } => {
                eprintln!("⚡ calling tool: {}", name);
            }
            QueryEvent::ToolResult {
                is_error, content, ..
            } => {
                if *is_error {
                    eprintln!("❌ tool error: {}", truncate(content, 200));
                }
            }
            _ => {}
        },
    })
}

fn handle_event_text(event: QueryEvent) {
    match event {
        QueryEvent::TextDelta(text) => {
            print!("{}", text);
            let _ = io::stdout().flush();
        }
        QueryEvent::ToolUseStart { name, .. } => {
            eprintln!("\n⚡ calling tool: {}", name);
        }
        QueryEvent::ToolResult {
            is_error, content, ..
        } => {
            if is_error {
                eprintln!("❌ tool error: {}", truncate(&content, 200));
            } else {
                eprintln!("✅ tool done ({})", byte_summary(&content));
            }
        }
        QueryEvent::TurnComplete { .. } => {
            println!();
        }
        QueryEvent::Usage {
            input_tokens,
            output_tokens,
            ..
        } => {
            eprintln!("  [tokens: {} in / {} out]", input_tokens, output_tokens);
        }
    }
}

fn handle_event_stream_json(event: QueryEvent) {
    let object = match event {
        QueryEvent::TextDelta(text) => {
            serde_json::json!({ "type": "text_delta", "text": text })
        }
        QueryEvent::ToolUseStart { id, name } => {
            serde_json::json!({ "type": "tool_use_start", "id": id, "name": name })
        }
        QueryEvent::ToolResult {
            tool_use_id,
            content,
            is_error,
        } => {
            serde_json::json!({
                "type": "tool_result",
                "tool_use_id": tool_use_id,
                "content": content,
                "is_error": is_error,
            })
        }
        QueryEvent::TurnComplete { stop_reason } => {
            serde_json::json!({ "type": "turn_complete", "stop_reason": format!("{stop_reason:?}") })
        }
        QueryEvent::Usage {
            input_tokens,
            output_tokens,
            cache_creation_input_tokens,
            cache_read_input_tokens,
        } => {
            serde_json::json!({
                "type": "usage",
                "input_tokens": input_tokens,
                "output_tokens": output_tokens,
                "cache_creation_input_tokens": cache_creation_input_tokens,
                "cache_read_input_tokens": cache_read_input_tokens,
            })
        }
    };
    println!("{object}");
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let mut end = max;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", &s[..end])
    }
}

fn byte_summary(s: &str) -> String {
    let len = s.len();
    if len < 1024 {
        format!("{len} bytes")
    } else {
        format!("{:.1} KB", len as f64 / 1024.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_ascii_within_limit() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_ascii_at_limit() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn truncate_ascii_over_limit() {
        assert_eq!(truncate("hello world", 5), "hello...");
    }

    #[test]
    fn truncate_multibyte_at_char_boundary() {
        assert_eq!(truncate("café", 4), "caf...");
    }

    #[test]
    fn truncate_multibyte_inside_char() {
        assert_eq!(truncate("a中b", 2), "a...");
    }

    #[test]
    fn truncate_cjk_string() {
        assert_eq!(truncate("你好世界", 7), "你好...");
    }

    #[test]
    fn truncate_emoji() {
        assert_eq!(truncate("hi😀bye", 4), "hi...");
    }

    #[test]
    fn truncate_japanese() {
        assert_eq!(truncate("こんにちは", 8), "こん...");
    }

    #[test]
    fn truncate_mixed_cjk_error_output() {
        let input = "error[E0308]: エラー: 型が一致しません expected `i32`, found `&str`";
        let result = truncate(input, 30);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 33 + 3);
    }

    #[test]
    fn truncate_empty() {
        assert_eq!(truncate("", 10), "");
    }
}
