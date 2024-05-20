use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    println!("Starting flights-data...");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("flights-data requires exactly 1 argument (`migrate` or `harvest`)");
    }
    let configuration = flights_data::configuration::get_configuration().unwrap();

    match args[1].as_str() {
        "migrate" => {
            println!("Running migrations...");
            let pool = PgPoolOptions::new()
                .max_connections(1)
                .connect(&configuration.database_url)
                .await
                .unwrap();

            sqlx::migrate!().run(&pool).await.unwrap();
        }
        "harvest" => {
            println!("Running data harvest...");
            flights_data::data_harvest::executor::run(&configuration).await
        }
        other => panic!(
            "Provided argument unknown: must be `migrate` or `harvest`, but was {}",
            other
        ),
    }
}
