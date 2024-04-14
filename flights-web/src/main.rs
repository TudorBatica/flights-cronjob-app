use sqlx::PgPool;
use std::net::TcpListener;

use flights_web::configuration;
use flights_web::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration();
    let pool = PgPool::connect(&configuration.database_url).await.unwrap();
    let addr = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(addr).unwrap();
    run(listener, pool)?.await
}
