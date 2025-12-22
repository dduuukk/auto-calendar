use anyhow::Result;
use std::env;

pub struct GCalConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl GCalConfig {
    pub async fn new() -> Result<GCalConfig> {
        Ok(GCalConfig {
            client_id: env::var("GCAL_CLIENT_ID")?,
            client_secret: env::var("GCAL_CLIENT_SECRET")?,
            redirect_uri: env::var("GCAL_REDIRECT_URI")?,
        })
    }
}

// Helper functions
async fn get_env_var_or_empty(key: &str) -> Result<String> {
    let result = match env::var(key) {
        Ok(result) => result,
        Err(_) => {
            log::info!("{key} does not exist or is mangled. Initializing to empty!");
            "".to_string()
        }
    };
    Ok(result)
}
