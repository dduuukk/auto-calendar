use anyhow::Result;
use std::env;
use todoist_v2_rest::TodoistUser;

pub struct TodoistConfig {
    user: TodoistUser,
}

impl TodoistConfig {
    pub async fn new() -> Result<TodoistConfig> {
        let api_token = env::var("TODOIST_API_TOKEN")?;

        Ok(TodoistConfig {
            user: TodoistUser::new(&api_token),
        })
    }
}
