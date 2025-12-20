pub mod gcal_config;
pub mod todoist_config;

use anyhow::Result;

use gcal_config::GCalConfig;
use todoist_config::TodoistConfig;

pub struct EnvConfig {
    gcal_config: GCalConfig,
    todoist_config: TodoistConfig,
}

impl EnvConfig {
    pub async fn new() -> Result<EnvConfig> {
        Ok(EnvConfig {
            gcal_config: GCalConfig::new().await?,
            todoist_config: TodoistConfig::new().await?,
        })
    }
}
