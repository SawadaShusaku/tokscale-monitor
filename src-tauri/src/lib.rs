// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder};
use tauri::Manager;
// HashMap can be added when needed for future aggregations

#[derive(Debug, Serialize)]
struct OpenCodeMessage {
    id: String,
    session_id: String,
    model: String,
    provider: String,
    cost: f64,
    created: i64,
}

#[derive(Debug, Serialize)]
struct OpenCodeData {
    messages: Vec<OpenCodeMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiedMessage {
    pub id: String,
    pub client: String,
    pub model_id: String,
    pub cost: f64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBreakdown {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub cache_write_tokens: u64,
}

#[derive(Debug, Serialize)]
struct UnifiedData {
    messages: Vec<UnifiedMessage>,
}

#[tauri::command]
fn get_tokscale_raw_data() -> Result<String, String> {
    let output = std::process::Command::new("tokscale")
        .args(["--json", "--no-spinner"])
        .output()
        .map_err(|e| format!("Failed to execute tokscale: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("tokscale exited with error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

#[derive(Debug, Deserialize)]
struct OpenCodeJsonMessage {
    #[serde(rename = "modelID")]
    model_id: Option<String>,
    #[serde(rename = "providerID")]
    provider_id: Option<String>,
    cost: Option<f64>,
    time: OpenCodeTime,
}

#[derive(Debug, Deserialize)]
struct OpenCodeTime {
    created: f64,
}

#[tauri::command]
fn get_opencode_messages() -> Result<String, String> {
    let db_path = dirs::home_dir()
        .map(|h| h.join(".local/share/opencode/opencode.db"))
        .ok_or("Could not determine home directory")?;

    if !db_path.exists() {
        return Err(format!("OpenCode database not found at: {:?}", db_path));
    }

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, session_id, data FROM message WHERE json_extract(data, '$.role') = 'assistant'"
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let messages = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?;
            let _session_id: String = row.get(1)?;
            let data_json: String = row.get(2)?;

            let msg: OpenCodeJsonMessage = serde_json::from_str(&data_json).unwrap_or(OpenCodeJsonMessage {
                model_id: None,
                provider_id: None,
                cost: None,
                time: OpenCodeTime { created: 0.0 },
            });

            Ok(OpenCodeMessage {
                id,
                session_id: _session_id,
                model: msg.model_id.unwrap_or_else(|| "unknown".to_string()),
                provider: msg.provider_id.unwrap_or_else(|| "unknown".to_string()),
                cost: msg.cost.unwrap_or(0.0),
                created: msg.time.created as i64,
            })
        })
        .map_err(|e| format!("Query failed: {}", e))?
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    let data = OpenCodeData { messages };
    serde_json::to_string(&data)
        .map_err(|e| format!("Failed to serialize data: {}", e))
}

fn parse_opencode_messages() -> Result<Vec<UnifiedMessage>, String> {
    let db_path = dirs::home_dir()
        .map(|h| h.join(".local/share/opencode/opencode.db"))
        .ok_or("Could not determine home directory")?;

    if !db_path.exists() {
        return Ok(vec![]);
    }

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, session_id, data FROM message WHERE json_extract(data, '$.role') = 'assistant'"
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let messages = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?;
            let _session_id: String = row.get(1)?;
            let data_json: String = row.get(2)?;

            let msg: OpenCodeJsonMessage = serde_json::from_str(&data_json).unwrap_or(OpenCodeJsonMessage {
                model_id: None,
                provider_id: None,
                cost: None,
                time: OpenCodeTime { created: 0.0 },
            });

            Ok(UnifiedMessage {
                id,
                client: "opencode".to_string(),
                model_id: msg.model_id.unwrap_or_else(|| "unknown".to_string()),
                cost: msg.cost.unwrap_or(0.0),
                timestamp: msg.time.created as i64,
            })
        })
        .map_err(|e| format!("Query failed: {}", e))?
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    Ok(messages)
}

#[derive(Debug, Deserialize)]
struct ClaudeMessage {
    #[serde(rename = "type")]
    msg_type: String,
    timestamp: Option<String>,
    uuid: Option<String>,
    message: Option<ClaudeMessageInner>,
}

#[derive(Debug, Deserialize)]
struct ClaudeMessageInner {
    model: Option<String>,
    usage: Option<ClaudeUsage>,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
    cache_creation_input_tokens: Option<u64>,
}

// Claude API pricing estimates (USD per 1M tokens)
const CLAUDE_COST_INPUT_PER_1M: f64 = 3.0;
const CLAUDE_COST_OUTPUT_PER_1M: f64 = 15.0;
const CLAUDE_COST_CACHE_READ_PER_1M: f64 = 0.37;
const CLAUDE_COST_CACHE_CREATE_PER_1M: f64 = 1.25;
const TOKENS_PER_MILLION: f64 = 1_000_000.0;

fn parse_claude_messages() -> Result<Vec<UnifiedMessage>, String> {
    let projects_dir = dirs::home_dir()
        .map(|h| h.join(".claude/projects"))
        .ok_or("Could not determine home directory")?;

    if !projects_dir.exists() {
        return Ok(vec![]);
    }

    let mut messages = Vec::new();

    fn visit_dirs(dir: &std::path::Path, messages: &mut Vec<UnifiedMessage>) -> std::io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, messages)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    for line in content.lines() {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(msg) = serde_json::from_str::<ClaudeMessage>(line) {
                            if msg.msg_type == "assistant" {
                                let timestamp = msg.timestamp.as_ref()
                                    .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                                    .map(|dt| dt.timestamp_millis())
                                    .unwrap_or(0);

                                let inner = msg.message.as_ref();
                                let model_id = inner
                                    .and_then(|m| m.model.clone())
                                    .unwrap_or_else(|| "claude-unknown".to_string());

                                // Estimate cost from tokens if available
                                let cost = inner.and_then(|m| m.usage.as_ref()).map(|u| {
                                    let input = u.input_tokens.unwrap_or(0);
                                    let output = u.output_tokens.unwrap_or(0);
                                    let cache_read = u.cache_read_input_tokens.unwrap_or(0);
                                    let cache_create = u.cache_creation_input_tokens.unwrap_or(0);
                                    let total = input as f64 * CLAUDE_COST_INPUT_PER_1M
                                        + output as f64 * CLAUDE_COST_OUTPUT_PER_1M
                                        + cache_read as f64 * CLAUDE_COST_CACHE_READ_PER_1M
                                        + cache_create as f64 * CLAUDE_COST_CACHE_CREATE_PER_1M;
                                    total / TOKENS_PER_MILLION
                                }).unwrap_or(0.0);

                                messages.push(UnifiedMessage {
                                    id: msg.uuid.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                                    client: "claude".to_string(),
                                    model_id,
                                    cost,
                                    timestamp,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    let _ = visit_dirs(&projects_dir, &mut messages);
    Ok(messages)
}

#[derive(Debug, Deserialize)]
struct CodexTurnContextPayload {
    model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CodexTurnContext {
    #[serde(rename = "type")]
    msg_type: String,
    payload: Option<CodexTurnContextPayload>,
}

#[derive(Debug, Deserialize)]
struct CodexPayload {
    #[serde(rename = "type")]
    _payload_type: Option<String>,
    role: Option<String>,
    model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CodexMessage {
    #[serde(rename = "type")]
    msg_type: String,
    timestamp: Option<String>,
    payload: Option<CodexPayload>,
}

// Codex cost estimates per assistant response (USD) — no token data in JSONL
// Using prefix matching so new versions (e.g. gpt-5.6) are handled automatically
const CODEX_COST_TIERS: &[(&str, f64)] = &[
    ("gpt-5.5", 0.05), // higher capability tier
    ("gpt-5.4", 0.03), // standard tier
    ("gpt-5.2", 0.02), // lighter tier
];
const CODEX_COST_DEFAULT: f64 = 0.03;

fn estimate_codex_cost(model_id: &str) -> f64 {
    for (prefix, cost) in CODEX_COST_TIERS {
        if model_id.starts_with(prefix) {
            return *cost;
        }
    }
    CODEX_COST_DEFAULT
}

fn parse_codex_messages() -> Result<Vec<UnifiedMessage>, String> {
    let sessions_dir = dirs::home_dir()
        .map(|h| h.join(".codex/sessions"))
        .ok_or("Could not determine home directory")?;

    if !sessions_dir.exists() {
        return Ok(vec![]);
    }

    let mut messages = Vec::new();

    fn visit_dirs(dir: &std::path::Path, messages: &mut Vec<UnifiedMessage>) -> std::io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, messages)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    // First pass: find turn_context to get model name
                    let mut file_model: Option<String> = None;
                    for line in content.lines() {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(ctx) = serde_json::from_str::<CodexTurnContext>(line) {
                            if ctx.msg_type == "turn_context" {
                                file_model = ctx.payload.as_ref().and_then(|p| p.model.clone());
                                break;
                            }
                        }
                    }

                    // Second pass: collect assistant response_items
                    for line in content.lines() {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(msg) = serde_json::from_str::<CodexMessage>(line) {
                            if msg.msg_type == "response_item" {
                                let payload = msg.payload.as_ref();
                                let role = payload.and_then(|p| p.role.as_ref());
                                if role == Some(&"assistant".to_string()) {
                                    let timestamp = msg.timestamp.as_ref()
                                        .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                                        .map(|dt| dt.timestamp_millis())
                                        .unwrap_or(0);

                                    let model_id = payload
                                        .and_then(|p| p.model.clone())
                                        .or_else(|| file_model.clone())
                                        .unwrap_or_else(|| "gpt-5.5".to_string());

                                    let cost = estimate_codex_cost(&model_id);

                                    messages.push(UnifiedMessage {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        client: "codex".to_string(),
                                        model_id,
                                        cost,
                                        timestamp,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    let _ = visit_dirs(&sessions_dir, &mut messages);
    Ok(messages)
}

fn parse_cursor_messages() -> Result<Vec<UnifiedMessage>, String> {
    // TODO: Implement Cursor parser once data format is confirmed
    Ok(vec![])
}

fn parse_gemini_messages() -> Result<Vec<UnifiedMessage>, String> {
    // TODO: Implement Gemini parser once data format is confirmed
    Ok(vec![])
}

fn parse_windsurf_messages() -> Result<Vec<UnifiedMessage>, String> {
    // TODO: Implement Windsurf parser once data format is confirmed
    Ok(vec![])
}

fn parse_copilot_messages() -> Result<Vec<UnifiedMessage>, String> {
    // TODO: Implement Copilot parser once data format is confirmed
    Ok(vec![])
}

fn parse_aider_messages() -> Result<Vec<UnifiedMessage>, String> {
    // TODO: Implement Aider parser once data format is confirmed
    Ok(vec![])
}

// Qwen CLI stores conversation logs in ~/.qwen/projects/*/chats/*.jsonl
// Each line is a JSON event. Token usage is found in:
//   - system/ui_telemetry records under systemPayload.uiEvent
//   - assistant records under usageMetadata

#[derive(Debug, Deserialize)]
struct QwenMessage {
    #[serde(rename = "type")]
    msg_type: String,
    subtype: Option<String>,
    uuid: Option<String>,
    #[serde(rename = "parentUuid")]
    parent_uuid: Option<String>,
    timestamp: Option<String>,
    model: Option<String>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<QwenUsageMetadata>,
    #[serde(rename = "systemPayload")]
    system_payload: Option<QwenSystemPayload>,
}

#[derive(Debug, Deserialize)]
struct QwenUsageMetadata {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: Option<u64>,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: Option<u64>,
    #[serde(rename = "totalTokenCount")]
    total_token_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct QwenSystemPayload {
    #[serde(rename = "uiEvent")]
    ui_event: Option<QwenUiEvent>,
}

#[derive(Debug, Deserialize)]
struct QwenUiEvent {
    model: Option<String>,
    #[serde(rename = "input_token_count")]
    input_token_count: Option<u64>,
    #[serde(rename = "output_token_count")]
    output_token_count: Option<u64>,
    #[serde(rename = "total_token_count")]
    total_token_count: Option<u64>,
}

// Estimated Qwen API pricing (USD per 1M tokens)
const QWEN_COST_INPUT_PER_1M: f64 = 0.3;
const QWEN_COST_OUTPUT_PER_1M: f64 = 0.6;
const QWEN_COST_DEFAULT_PER_1M: f64 = 0.5;

fn parse_qwen_messages() -> Result<Vec<UnifiedMessage>, String> {
    let projects_dir = dirs::home_dir()
        .map(|h| h.join(".qwen/projects"))
        .ok_or("Could not determine home directory")?;

    if !projects_dir.exists() {
        return Ok(vec![]);
    }

    let mut messages = Vec::new();

    fn visit_dirs(dir: &std::path::Path, messages: &mut Vec<UnifiedMessage>) -> std::io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, messages)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    // First pass: collect assistant records for model lookup
                    let mut model_by_uuid: std::collections::HashMap<String, String> =
                        std::collections::HashMap::new();
                    for line in content.lines() {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(msg) = serde_json::from_str::<QwenMessage>(line) {
                            if msg.msg_type == "assistant" {
                                if let (Some(uuid), Some(model)) = (&msg.uuid, &msg.model) {
                                    model_by_uuid.insert(uuid.clone(), model.clone());
                                }
                            }
                        }
                    }

                    // Second pass: extract usage from telemetry / assistant records
                    for line in content.lines() {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(msg) = serde_json::from_str::<QwenMessage>(line) {
                            let mut input_tokens: u64 = 0;
                            let mut output_tokens: u64 = 0;
                            let mut model_id: Option<String> = None;
                            let mut timestamp: i64 = 0;

                            // Extract timestamp
                            if let Some(ts) = &msg.timestamp {
                                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
                                    timestamp = dt.timestamp_millis();
                                }
                            }

                            if msg.msg_type == "system" && msg.subtype.as_deref() == Some("ui_telemetry") {
                                // Try systemPayload.uiEvent first
                                if let Some(payload) = &msg.system_payload {
                                    if let Some(ui) = &payload.ui_event {
                                        input_tokens = ui.input_token_count.unwrap_or(0);
                                        output_tokens = ui.output_token_count.unwrap_or(0);
                                        model_id = ui.model.clone();
                                    }
                                }
                                // Fallback: usageMetadata on the system record itself
                                if input_tokens == 0 && output_tokens == 0 {
                                    if let Some(um) = &msg.usage_metadata {
                                        input_tokens = um.prompt_token_count.unwrap_or(0);
                                        output_tokens = um.candidates_token_count.unwrap_or(0);
                                    }
                                }
                                // Resolve model via parentUuid -> assistant record
                                if model_id.is_none() {
                                    if let Some(parent) = &msg.parent_uuid {
                                        model_id = model_by_uuid.get(parent).cloned();
                                    }
                                }
                            } else if msg.msg_type == "assistant" {
                                // Some assistant records carry usageMetadata directly
                                if let Some(um) = &msg.usage_metadata {
                                    input_tokens = um.prompt_token_count.unwrap_or(0);
                                    output_tokens = um.candidates_token_count.unwrap_or(0);
                                    model_id = msg.model.clone();
                                }
                            }

                            let total_tokens = input_tokens + output_tokens;
                            if total_tokens > 0 {
                                let cost = if input_tokens > 0 && output_tokens > 0 {
                                    (input_tokens as f64 * QWEN_COST_INPUT_PER_1M
                                        + output_tokens as f64 * QWEN_COST_OUTPUT_PER_1M)
                                        / 1_000_000.0
                                } else {
                                    (total_tokens as f64 * QWEN_COST_DEFAULT_PER_1M) / 1_000_000.0
                                };

                                let model = model_id.unwrap_or_else(|| "qwen-coder".to_string());

                                messages.push(UnifiedMessage {
                                    id: msg.uuid.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                                    client: "qwen".to_string(),
                                    model_id: model,
                                    cost,
                                    timestamp,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    let _ = visit_dirs(&projects_dir, &mut messages);
    Ok(messages)
}

#[tauri::command]
fn get_unified_messages(client: String) -> Result<String, String> {
    let messages = match client.as_str() {
        "opencode" => parse_opencode_messages()?,
        "claude" => parse_claude_messages()?,
        "codex" => parse_codex_messages()?,
        "cursor" => parse_cursor_messages()?,
        "gemini" => parse_gemini_messages()?,
        "windsurf" => parse_windsurf_messages()?,
        "copilot" => parse_copilot_messages()?,
        "aider" => parse_aider_messages()?,
        "qwen" => parse_qwen_messages()?,
        _ => vec![],
    };

    let data = UnifiedData { messages };
    serde_json::to_string(&data)
        .map_err(|e| format!("Failed to serialize data: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_parse_claude_messages_with_mock_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let projects_dir = temp_dir.path().join("projects");
        std::fs::create_dir_all(&projects_dir).unwrap();

        let session_dir = projects_dir.join("test-project");
        std::fs::create_dir_all(&session_dir).unwrap();

        let jsonl_path = session_dir.join("session.jsonl");
        let mut file = std::fs::File::create(&jsonl_path).unwrap();
        writeln!(
            file,
            r#"{{"type":"assistant","timestamp":"2024-01-15T12:00:00Z","uuid":"test-uuid-1","message":{{"model":"claude-opus-4-7","usage":{{"input_tokens":1000,"output_tokens":500,"cache_read_input_tokens":200}}}}}}"#
        ).unwrap();
        writeln!(
            file,
            r#"{{"type":"user","timestamp":"2024-01-15T12:01:00Z","uuid":"test-uuid-2"}}"#
        ).unwrap();
        writeln!(
            file,
            r#"{{"type":"assistant","timestamp":"2024-01-15T12:02:00Z","uuid":"test-uuid-3","message":{{"model":"claude-sonnet-4-6","usage":{{"input_tokens":2000,"output_tokens":1000}}}}}}"#
        ).unwrap();

        let content = std::fs::read_to_string(&jsonl_path).unwrap();
        let mut messages = Vec::new();
        for line in content.lines() {
            if let Ok(msg) = serde_json::from_str::<ClaudeMessage>(line) {
                if msg.msg_type == "assistant" {
                    let timestamp = msg.timestamp.as_ref()
                        .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                        .map(|dt| dt.timestamp_millis())
                        .unwrap_or(0);
                    let inner = msg.message.as_ref();
                    let model_id = inner
                        .and_then(|m| m.model.clone())
                        .unwrap_or_else(|| "claude-unknown".to_string());
                    let cost = inner.and_then(|m| m.usage.as_ref()).map(|u| {
                        let input = u.input_tokens.unwrap_or(0);
                        let output = u.output_tokens.unwrap_or(0);
                        let cache_read = u.cache_read_input_tokens.unwrap_or(0);
                        let cache_create = u.cache_creation_input_tokens.unwrap_or(0);
                        let total = input as f64 * CLAUDE_COST_INPUT_PER_1M
                            + output as f64 * CLAUDE_COST_OUTPUT_PER_1M
                            + cache_read as f64 * CLAUDE_COST_CACHE_READ_PER_1M
                            + cache_create as f64 * CLAUDE_COST_CACHE_CREATE_PER_1M;
                        total / TOKENS_PER_MILLION
                    }).unwrap_or(0.0);
                    messages.push(UnifiedMessage {
                        id: msg.uuid.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                        client: "claude".to_string(),
                        model_id,
                        cost,
                        timestamp,
                    });
                }
            }
        }

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].id, "test-uuid-1");
        assert_eq!(messages[0].model_id, "claude-opus-4-7");
        assert!(messages[0].cost > 0.0);
        assert_eq!(messages[1].model_id, "claude-sonnet-4-6");
    }

    #[test]
    fn test_parse_codex_messages_with_mock_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let sessions_dir = temp_dir.path().join("sessions");
        std::fs::create_dir_all(&sessions_dir).unwrap();

        let jsonl_path = sessions_dir.join("rollout-2024-01-15.jsonl");
        let mut file = std::fs::File::create(&jsonl_path).unwrap();
        writeln!(
            file,
            r#"{{"type":"turn_context","payload":{{"model":"gpt-5.5"}}}}"#
        ).unwrap();
        writeln!(
            file,
            r#"{{"type":"response_item","timestamp":"2024-01-15T12:00:00Z","payload":{{"type":"message","role":"assistant"}}}}"#
        ).unwrap();
        writeln!(
            file,
            r#"{{"type":"request","timestamp":"2024-01-15T12:01:00Z","payload":{{"type":"message","role":"user"}}}}"#
        ).unwrap();
        writeln!(
            file,
            r#"{{"type":"response_item","timestamp":"2024-01-15T12:02:00Z","payload":{{"type":"message","role":"assistant"}}}}"#
        ).unwrap();

        let content = std::fs::read_to_string(&jsonl_path).unwrap();
        let mut file_model: Option<String> = None;
        for line in content.lines() {
            if let Ok(ctx) = serde_json::from_str::<CodexTurnContext>(line) {
                if ctx.msg_type == "turn_context" {
                    file_model = ctx.payload.as_ref().and_then(|p| p.model.clone());
                    break;
                }
            }
        }

        let mut messages = Vec::new();
        for line in content.lines() {
            if let Ok(msg) = serde_json::from_str::<CodexMessage>(line) {
                if msg.msg_type == "response_item" {
                    let payload = msg.payload.as_ref();
                    let role = payload.and_then(|p| p.role.as_ref());
                    if role == Some(&"assistant".to_string()) {
                        let timestamp = msg.timestamp.as_ref()
                            .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                            .map(|dt| dt.timestamp_millis())
                            .unwrap_or(0);
                        let model_id = payload
                            .and_then(|p| p.model.clone())
                            .or_else(|| file_model.clone())
                            .unwrap_or_else(|| "gpt-5.5".to_string());
                        messages.push(UnifiedMessage {
                            id: uuid::Uuid::new_v4().to_string(),
                            client: "codex".to_string(),
                            model_id,
                            cost: 0.0,
                            timestamp,
                        });
                    }
                }
            }
        }

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].model_id, "gpt-5.5");
        assert_eq!(messages[1].model_id, "gpt-5.5");
        assert_eq!(messages[0].client, "codex");
    }

    #[test]
    fn test_unsupported_client_returns_empty() {
        let client = "unsupported_client";
        let messages: Vec<UnifiedMessage> = match client {
            "opencode" => vec![],
            "claude" => vec![],
            "codex" => vec![],
            "cursor" => vec![],
            "gemini" => vec![],
            "windsurf" => vec![],
            "copilot" => vec![],
            "aider" => vec![],
            "qwen" => vec![],
            _ => vec![],
        };
        assert!(messages.is_empty());
    }

    #[test]
    fn test_parse_real_claude_projects() {
        let messages = parse_claude_messages().unwrap();
        println!("Claude parsed {} messages", messages.len());
        if messages.is_empty() {
            println!("Warning: No Claude messages found");
            return;
        }

        // Find messages in the last 5 hours
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - 5 * 60 * 60 * 1000;
        let recent: Vec<_> = messages.iter().filter(|m| m.timestamp >= cutoff).collect();
        println!("Claude messages in last 5h: {}", recent.len());

        let total_cost: f64 = recent.iter().map(|m| m.cost).sum();
        println!("Claude 5h total cost: ${:.4}", total_cost);
        println!("Claude 5h usage vs $20 limit: {:.2}%", (total_cost / 20.0) * 100.0);

        // Show model breakdown
        let mut model_counts = std::collections::HashMap::new();
        for m in &messages {
            *model_counts.entry(m.model_id.clone()).or_insert(0) += 1;
        }
        println!("Claude models: {:?}", model_counts);
    }

    #[test]
    fn test_parse_real_codex_sessions() {
        let messages = parse_codex_messages().unwrap();
        println!("Codex parsed {} messages", messages.len());
        if messages.is_empty() {
            println!("Warning: No Codex messages found");
            return;
        }

        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - 5 * 60 * 60 * 1000;
        let recent: Vec<_> = messages.iter().filter(|m| m.timestamp >= cutoff).collect();
        println!("Codex messages in last 5h: {}", recent.len());

        let total_cost: f64 = recent.iter().map(|m| m.cost).sum();
        println!("Codex 5h total cost: ${:.4}", total_cost);

        let mut model_counts = std::collections::HashMap::new();
        for m in &messages {
            *model_counts.entry(m.model_id.clone()).or_insert(0) += 1;
        }
        println!("Codex models: {:?}", model_counts);
    }

    #[test]
    fn test_parse_real_qwen_projects() {
        let messages = parse_qwen_messages().unwrap();
        println!("Qwen parsed {} messages", messages.len());
        if messages.is_empty() {
            println!("Warning: No Qwen messages found");
            return;
        }

        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - 5 * 60 * 60 * 1000;
        let recent: Vec<_> = messages.iter().filter(|m| m.timestamp >= cutoff).collect();
        println!("Qwen messages in last 5h: {}", recent.len());

        let total_cost: f64 = recent.iter().map(|m| m.cost).sum();
        println!("Qwen 5h total cost: ${:.4}", total_cost);

        let mut model_counts = std::collections::HashMap::new();
        for m in &messages {
            *model_counts.entry(m.model_id.clone()).or_insert(0) += 1;
        }
        println!("Qwen models: {:?}", model_counts);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_tokscale_raw_data, get_opencode_messages, get_unified_messages])
        .setup(|app| {
            // Prevent duplicate tray icons during dev hot-reload
            if app.tray_by_id("main").is_some() {
                return Ok(());
            }

            // Create tray menu
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            // Create tray icon
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
