#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub kiwi_api_key: String,
    pub monitored_period_length_days: u16,
    pub hours_between_scans: i64,
    pub minutes_between_cron_jobs: u64,
    pub db_new_routes_channel: String,
}

pub fn get_configuration() -> Settings {
    dotenvy::dotenv().ok();

    let config_file = config::File::with_name("config.yaml");
    let env = config::Environment::default();
    let config = config::Config::builder()
        .add_source(config_file)
        .add_source(env)
        .build()
        .unwrap();

    config.try_deserialize().unwrap()
}
