use anyhow::Result;
use std::env;
use tokio_postgres::{Client, NoTls};

pub struct PostgresConfig {
    url: String, // TODO: remove this?
    client: Client,
}

impl PostgresConfig {
    pub async fn new() -> Result<PostgresConfig> {
        let url = env::var("DATABASE_URL")?;

        let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log::error!("Database connection error: {e:?}")
            }
        });

        Ok(PostgresConfig { url, client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
