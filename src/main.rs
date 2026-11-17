mod config;
mod types;
mod client;

use client::LlmClient;
use config::{Config, PermissionMode};
use types::{Message};

fn main(){
    let cfg = Config::new(
        "http://127.0.0.1:8080/v1",
        "qwen2.5-coder:7b",
        10,
        PermissionMode::AskBeforeWrite,
    );

    let client = LlmClient::new();

    let transcript = vec![
        Message::system("You are a coding assistant. Always use the provided tools when asked to inspect files."),
        Message::user("Please read Cargo.toml"),
    ];

    println!("Sending request to {} at {}", cfg.model, cfg.base_url);

    match client.chat(&cfg, &transcript){
        Ok(reply)=>{
            if let Some(text) = &reply.content{
                if !text.is_empty(){
                    println!("\nAssistant text:\n{}", text);
                }
            }

            if let Some(calls) = &reply.tool_calls{
                for call in calls{
                    match call.parse_action(){
                        Ok(action)=> println!(
                            "\nModel requested valid tool: {:?} (needs approval: {})",
                            action,
                            action.needs_permission()
                        ),
                        Err(reason)=> println!("\nRefused malformed tool call: {}", reason),
                    }
                }
            }
        }
        Err(err)=> println!("\nError talking to server: {}", err),
    }
}