#[derive(Debug)]
pub enum PermissionMode{
    ReadOnly,
    AskBeforeWrite,
}

#[derive(Debug)]
pub struct Config{
    pub base_url: String,
    pub model: String,
    pub max_steps: usize,
    pub permission: PermissionMode,
}

impl Config{
    pub fn new(
        base_url: &str,
        model: &str,
        max_steps: usize,
        permission: PermissionMode,
    )->Self{
        Self{
            base_url: base_url.to_string(),
            model: model.to_string(),
            max_steps,
            permission,
        }
    }
}