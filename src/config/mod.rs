pub mod database_config;
pub mod gcal_config;
pub mod todoist_config;
pub mod user_config;

use anyhow::Result;

use database_config::PostgresConfig;
use gcal_config::GCalConfig;
use todoist_config::TodoistConfig;
use user_config::UserConfig;

pub struct EnvConfig {
    pub gcal_config: GCalConfig,
    pub todoist_config: TodoistConfig,
    pub pg_config: PostgresConfig,
    pub user_config: UserConfig,
}

impl EnvConfig {
    pub async fn new() -> Result<EnvConfig> {
        Ok(EnvConfig {
            gcal_config: GCalConfig::new().await?,
            todoist_config: TodoistConfig::new().await?,
            pg_config: PostgresConfig::new().await?,
            user_config: UserConfig::new().await?,
        })
    }
}
