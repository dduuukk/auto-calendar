use anyhow::Result;
use std::env;

pub struct GCalConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub token: String,
    pub refresh_token: String,
}

impl GCalConfig {
    pub async fn new() -> Result<GCalConfig> {
        Ok(GCalConfig {
            client_id: env::var("GCAL_CLIENT_ID")?,
            client_secret: env::var("GCAL_CLIENT_SECRET")?,
            redirect_uri: env::var("GCAL_REDIRECT_URI")?,
            token: get_env_var_or_empty("GCAL_TOKEN").await?,
            refresh_token: get_env_var_or_empty("GCAL_REFRESH_TOKEN").await?,
        })
    }
}

// Helper functions
async fn get_env_var_or_empty(key: &str) -> Result<String> {
    let result = match env::var(key) {
        Ok(result) => result,
        Err(_) => {
            log::info!(
                "{} does not exist or is mangled. Initializing to empty!",
                key.to_string()
            );
            "".to_string()
        }
    };
    Ok(result)
}
