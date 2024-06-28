use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    println!("Starting flights-data...");

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("flights-data can have at most 1 argument (`generate`)");
    }

    let configuration = flights_data::configuration::get_configuration().unwrap();

    if args.len() == 1 {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&configuration.database_url)
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        return;
    }

    match args[1].as_str() {
        "generate" => flights_data::migration::executor::run(&configuration).await,
        _ => panic!("Provided argument unknown: must be `generate"),
    }
}
