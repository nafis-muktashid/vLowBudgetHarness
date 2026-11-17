use std::time::Duration;

use serde::{Deserialize, Serialize};
use ureq::Agent;

use crate::config::Config;
use crate::types::{Message, tool_definitions};

#[derive(Serialize)]
struct ChatRequest<'a>{
    model: &'a str,
    messages: &'a [Message],
    tools: serde_json::Value,
}

#[derive(Deserialize)]
struct ChatResponse{
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice{
    message: Message,
}

pub struct LlmClient{
    agent:Agent,
}

impl LlmClient{
    pub fn new()-> Self{
        let config = Agent::config_builder().timeout_global(Some(Duration::from_secs(120))).http_status_as_error(false).build();

        Self{
            agent: Agent::new_with_config(config),
        }
    }

    pub fn chat(&self, cfg:&Config, messages:&[Message])->Result<Message, String>{
        let url = format!("{}/chat/completions", cfg.base_url.trim_end_matches('/'));

        let payload = ChatRequest{
            model: &cfg.model,
            messages,
            tools: tool_definitions(),
        };

        let mut response = self.agent.post(&url).send_json(&payload).map_err(|e| format!("Failed to reach server at {}: {}", url, e))?;

        let status = response.status();
        if !status.is_success(){
            let err_body = response.body_mut().read_to_string().unwrap_or_else(|_| "<unreadable error body>".to_string());
            return Err(format!("Server returned HTTP {}: {}", status, err_body));
        }

        let parsed: ChatResponse = response.body_mut().read_json().map_err(|e| format!("Failed to parse server JSON response: {}", e))?;

        let first = parsed.choices.into_iter().next().ok_or_else(|| "Server returned an empty response".to_string())?;

        Ok(first.message)
    }
}