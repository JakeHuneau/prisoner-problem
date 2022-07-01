use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub num_prisoners: usize,
    pub num_guesses: usize,
    pub num_runs: usize,
    pub debug: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("configuration/base"))
            .build()?;
        s.try_deserialize()
    }
}
