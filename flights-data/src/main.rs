use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    println!("Starting flights-data...");
    let configuration = flights_data::configuration::get_configuration().unwrap();
    flights_data::migration::executor::run(&configuration).await;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&configuration.database_url)
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
}
