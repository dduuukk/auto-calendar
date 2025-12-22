use anyhow::Result;
use std::env;
use todoist_v2_rest::TodoistUser;

pub struct UserConfig {
    email: String,
}

impl UserConfig {
    pub async fn new() -> Result<UserConfig> {
        let email = env::var("USER_EMAIL")?;

        Ok(UserConfig { email })
    }
}
