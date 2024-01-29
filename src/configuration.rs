//! src/configuration.rs
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    let settings = config::Config::builder()
    // Add configuration values from a file named `configuration.yaml`
    .add_source(
        config::File::new("configuration.yaml", config::FileFormat::Yaml)
    )
    .build()?;
    // Try to convert the configuration values it read into our Settings type
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    // We're using a PostgreSQL database, so we need to build a connection string
    // that looks like this:
    // postgresql://<username>:<password>@<host>:<port>/<database-name>
    // We're using the postgres crate to help us build the connection string.
    // We're also using the once_cell crate to make this a lazy static.
    // This means it will be initialized the first time it's used, and will then
    // hold onto the value so we don't need to recompute it every time.
    pub fn connection_string(&self) -> String {
        // We're using the postgres crate to help us build the connection string.
        // We're also using the once_cell crate to make this a lazy static.
        // This means it will be initialized the first time it's used, and will then
        // hold onto the value so we don't need to recompute it every time.
            format!(
                "postgresql://{}:{}@{}:{}/{}",
                self.username,
                self.password,
                self.host,
                self.port,
                self.database_name
            )
    }
}