use sqlx::PgPool;
use std::error::Error;
use std::result::Result;

mod api_client;
mod configuration;
mod monitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = configuration::get_configuration();
    let pool = PgPool::connect(&configuration.database_url).await.unwrap();
    monitor::run(&configuration, &pool).await;

    return Ok(());
}
