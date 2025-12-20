use anyhow::Result;

use google_calendar::Client;

use crate::config::gcal_config::GCalConfig;

pub struct GCalender {
    gcal_client: Client,
}

impl GCalender {
    pub async fn new(gcal_config: GCalConfig) -> Result<GCalender> {
        let mut gcal_client = Client::new(
            gcal_config.client_id,
            gcal_config.client_secret,
            gcal_config.redirect_uri,
            gcal_config.token,
            gcal_config.refresh_token,
        );

        Ok(GCalender { gcal_client })
    }
}
