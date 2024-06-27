use crate::configuration::Settings;
use flights_data::db_schema::Route;
use sqlx::postgres::PgListener;
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
    let db_listener = tokio::spawn(async move {
        run_db_listener(config_clone).await;
    });

    let _ = cron_task.await;
    let _ = db_listener.await;

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

async fn run_db_listener(configuration: Settings) {
    let pool = PgPool::connect(&configuration.database_url).await.unwrap();
    let mut listener = PgListener::connect_with(&pool).await.unwrap();
    listener
        .listen(&configuration.db_new_routes_channel)
        .await
        .unwrap();

    loop {
        let notification = listener.recv().await.unwrap();
        println!("Received notification from postgres");
        let route: Route = serde_json::from_str(notification.payload()).unwrap();
        monitor::run_for_routes(&configuration, &pool, vec![route]).await;
    }
}
