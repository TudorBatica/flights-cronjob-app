use crate::configuration::Settings;
use sqlx::PgPool;
use std::error::Error;
use std::result::Result;
use tokio::time::{sleep, Duration};

mod api_client;
mod configuration;
mod monitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = configuration::get_configuration();
    let config_clone = configuration.clone();

    let cron_task = tokio::spawn(async move {
        run_cron_monitoring(configuration).await;
    });

    let _ = cron_task.await;

    return Ok(());
}

async fn run_cron_monitoring(configuration: Settings) {
    let pool = PgPool::connect(&configuration.database_url).await.unwrap();
    loop {
        monitor::run(&configuration, &pool).await;
        sleep(Duration::from_secs(
            configuration.minutes_between_cron_jobs * 60,
        ))
        .await;
    }
}
