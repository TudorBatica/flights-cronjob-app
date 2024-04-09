use sqlx::postgres::PgPoolOptions;
use std::env;

mod configuration;

mod data_harvest {
    pub mod executor;
    mod locations_api_client;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("flights-data requires exactly 1 argument (`migrate` or `harvest`)");
    }

    let configuration = configuration::get_configuration().unwrap();
    match args[1].as_str() {
        "migrate" => {
            let pool = PgPoolOptions::new()
                .max_connections(1)
                .connect(&configuration.database.conn_string())
                .await
                .unwrap();

            sqlx::migrate!().run(&pool).await.unwrap();
        }
        "harvest" => data_harvest::executor::run(&configuration).await,
        _ => panic!("Provided argument unknown: must be `migrate` or `harvest`"),
    }
}
