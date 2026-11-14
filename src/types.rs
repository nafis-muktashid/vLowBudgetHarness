#[derive(Debug)]
pub enum Role{
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug)]
pub struct Message{
    pub role: Role,
    pub content: Option<String>,
}

impl Message{
    pub fn system(text: &str)->Self{
        Self{
            role: Role::System,
            content: Some(text.to_string()),
        }
    }

    pub fn user(text: &str)->Self{
        Self{
            role: Role::User,
            content: Some(text.to_string()),
        }
    }
}

#[derive(Debug)]
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