use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role{
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall{
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall{
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message{
    pub role: Role,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message{
    pub fn system(text: &str)->Self{
        Self{
            role: Role::System,
            content: Some(text.to_string()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(text: &str)->Self{
        Self{
            role: Role::User,
            content: Some(text.to_string()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn tool_result(tool_call_id: &str, output: &str)->Self{
        Self{
            role: Role::Tool,
            content: Some(output.to_string()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name", content = "args", rename_all = "snake_case")]
pub enum ToolAction{
    ReadFile{path: String},
    Search{pattern: String, path: String},
    ReplaceString{path: String, old: String, new: String},
    RunCommand{command: String},
}

impl ToolAction{
    pub fn needs_permission(&self)-> bool{
        match self{
            ToolAction::ReadFile{..} | ToolAction::Search{..}=> false,
            ToolAction::ReplaceString{..} | ToolAction::RunCommand{..}=> true,
        }
    }
}

impl ToolCall{
    pub fn parse_action(&self) -> Result<ToolAction, String>{
        let args: serde_json::Value = serde_json::from_str(&self.function.arguments)
            .map_err(|e| format!("Malformed JSON in arguments: {}", e))?;

        let combined = serde_json::json!({
            "name": self.function.name,
            "args": args,
        });

        serde_json::from_value(combined).map_err(|e| format!("Invalid tool call '{}': {}", self.function.name, e))
    }
}

pub fn tool_definitions()-> serde_json::Value{
    serde_json::json!([
        {
            "type": "function",
            "function": {
                "name": "read_file",
                "description": "Read the contents of a file",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "The path to the file to read"},
                    },
                    "required": ["path"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "search",
                "description": "Search for files matching a text pattern",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "pattern": {"type": "string", "description": "Text pattern to search for"},
                        "path": {"type": "string", "description": "File or directory path to search in"},
                    },
                    "required": ["pattern", "path"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "replace_string",
                "description": "Replace a string in a file",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "The path to the file to modify"},
                        "old": {"type": "string", "description": "The string to replace"},
                        "new": {"type": "string", "description": "The replacement string"},
                    },
                    "required": ["path", "old", "new"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "run_command",
                "description": "Execute a shell command in the project directory",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "command": {"type": "string", "description": "The command to execute"},
                    },
                    "required": ["command"]
                }
            }
        }
    ])
}