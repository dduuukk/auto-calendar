use anyhow::Result;

use crate::config::gcal_config::GCalConfig;

pub struct GCalender {
    gcal_client: google_calendar::Client,
}

impl GCalender {
    pub async fn new(
        gcal_config: GCalConfig,
        db_client: tokio_postgres::Client,
        email: String,
    ) -> Result<GCalender> {
        let email = email.as_str();
        let existing_user = db_client
            .query(
                "SELECT google_access_token, google_refresh_token FROM users WHERE email = $1",
                &[&email],
            )
            .await?;

        if let Some(row) = existing_user.first() {
            let token: Option<String> = row.get(0);
            let refresh_token: Option<String> = row.get(1);

            if let (Some(token), Some(refresh_token)) = (token, refresh_token) {
                let gcal_client = google_calendar::Client::new(
                    gcal_config.client_id,
                    gcal_config.client_secret,
                    gcal_config.redirect_uri,
                    token,
                    refresh_token,
                );

                return Ok(GCalender { gcal_client });
            }
        }

        // TODO: otherwise we have to insert into db and raw query with empty strings
        Ok(GCalender { gcal_client })
    }
}
