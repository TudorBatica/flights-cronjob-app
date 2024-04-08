#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DbConfig,
    pub kiwi_api_key: String,
}

#[derive(serde::Deserialize)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DbConfig {
    pub fn conn_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    dotenvy::dotenv().ok();

    let config_file = config::File::with_name("config.yaml");
    let env = config::Environment::with_prefix("FLIGHTS");
    let config = config::Config::builder()
        .add_source(config_file)
        .add_source(env)
        .build()?;

    config.try_deserialize()
}
