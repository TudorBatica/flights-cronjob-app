use sqlx::postgres::PgPoolOptions;

mod configuration;

#[tokio::main]
async fn main() {
    let configuration = configuration::get_configuration().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&configuration.database.conn_string())
        .await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();
}
