use sqlx::PgPool;
use std::net::TcpListener;

use flights_web::configuration;
use flights_web::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting flights-web...");
    let configuration = configuration::get_configuration();
    println!(
        "Trying to connect to database at {}",
        configuration.database_url
    );
    let pool = PgPool::connect(&configuration.database_url).await.unwrap();
    let addr = format!("0.0.0.0:{}", configuration.app_port);
    let listener = TcpListener::bind(addr).unwrap();
    run(listener, pool)?.await
}
