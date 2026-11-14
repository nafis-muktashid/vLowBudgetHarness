mod config;
mod types;

use config::{Config, PermissionMode};
use types::{Message, ToolAction};

fn main(){
    let cfg = Config::new(
        "http://127.0.0.1:8080/v1",
        "qwen2.5-coder:7b",
        10,
        PermissionMode::AskBeforeWrite,
    );

    let mut transcript = vec![
        Message::system("You are a helpful coding assistant."),
    ];
    transcript.push(Message::user("Please read Cargo.toml"));

    let sample_tool = ToolAction::ReadFile{
        path: "Cargo.toml".to_string(),
    };

    println!("Model: {} | Messages in chat: {}", cfg.model, transcript.len());
    println!("Needs user approval? {}", sample_tool.needs_permission());
    println!("Next tool to run: {:?}", sample_tool);
}