pub mod config;
mod gcal;

use config::EnvConfig;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = EnvConfig::new().await;
    if let Err(e) = config {
        log::error!("Config init error {e:?}")
    } else {
        log::error!("Config init without error");
    }
}
