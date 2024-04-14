#[derive(serde::Deserialize)]
pub struct Settings {
    pub app_port: u16,
    pub database_url: String,
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
