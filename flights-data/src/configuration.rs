#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DbConfig,
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
            self.username, self.password,
            self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config_file = config::File::with_name("config.yaml");
    let config = config::Config::builder()
        .add_source(config_file)
        .build()?;

    config.try_deserialize()
}

