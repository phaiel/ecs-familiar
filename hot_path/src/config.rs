use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub api_key: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");

        let s = config::Config::builder()
            // Start with hardcoded defaults
            .add_source(config::File::with_name(&format!("{}/config/default", manifest_dir)))
            // Add in a local configuration file to override defaults
            .add_source(config::File::with_name(&format!("{}/config/local", manifest_dir)).required(false))
            // Add in settings from the environment (with a prefix of FAMILIAR)
            // e.g. `FAMILIAR_API_KEY=123` would set the `api_key` config value.
            .add_source(config::Environment::with_prefix("FAMILIAR"))
            .build()?;

        s.try_deserialize()
    }
} 