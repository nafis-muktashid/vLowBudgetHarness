mod config;
mod types;

use config::{Config, PermissionMode};
use types::{Message, ToolCall};

fn main(){
    let cfg = Config::new(
        "http://127.0.0.1:8080/v1",
        "qwen2.5-coder:7b",
        10,
        PermissionMode::AskBeforeWrite,
    );

    let mut transcript = vec![
        Message::system("You are a helpful coding assistant."),
        Message::user("Please read Cargo.toml"),
    ];
 
    let json_payload = serde_json::to_string_pretty(&transcript).unwrap();
    println!("--- Outgoin JSON for {} ---\n{}\n", cfg.model, json_payload);

    // 1. Simulate a VALID tool CAll reply from the model
    let valid_reply = r#"{
        "id": "call_1",
        "type": "function",
        "function": {
            "name": "read_file",
            "arguments": "{\"path\": \"Cargo.toml\"}"
        }
    }"#;

    // 2. Simulate a MALFORMED tool call (missing required "path" field)
    let bad_reply = r#"{
        "id": "call_2",
        "type": "function",
        "function": {
            "name": "read_file",
            "arguments": "{\"wrong_field\": 123}"
        }
    }"#;

    for raw_json in [valid_reply, bad_reply]{
        let call: ToolCall = serde_json::from_str(raw_json).unwrap();
        match call.parse_action(){
            Ok(action) => {
                println!(
                    "Accepted: {:?} (needs approval: {})",
                    action, 
                    action.needs_permission()
                );
                transcript.push(Message::tool_result(&call.id, "file contents..."));
            }
            Err(reason) => {
                println!("Refused malformed call: {}", reason);
            }
        }
    }
}