use std::path::Path;

use reqwest::Url;
use serde::Deserialize;

pub const CONFIG_PREFIX: &str = "CTF_BOT";

#[derive(Debug, Clone, Deserialize)]
pub struct CtfConfig {
    /// Ethereum Signer
    pub private_key: String,
    /// Semaphore secret
    pub secret: String,
    /// WC Sepolia Provider
    pub provider: Url,
}

impl CtfConfig {
    pub fn load(config_path: Option<&Path>) -> eyre::Result<Self> {
        let mut settings = config::Config::builder();

        if let Some(path) = config_path {
            settings = settings.add_source(config::File::from(path).required(true));
        }

        let settings = settings
            .add_source(
                config::Environment::with_prefix(CONFIG_PREFIX)
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;

        let config = settings.try_deserialize::<Self>()?;

        Ok(config)
    }
}
