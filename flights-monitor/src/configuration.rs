#[derive(serde::Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub kiwi_api_key: String,
    pub monitored_period_length_days: u16,
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
