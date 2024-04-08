use sqlx::postgres::PgPoolOptions;

mod configuration;

mod data_harvest {
    pub mod executor;
    mod locations_api_client;
}

#[tokio::main]
async fn main() {
    let configuration = configuration::get_configuration().unwrap();

    data_harvest::executor::run().await;

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&configuration.database.conn_string())
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();
}
