use std::path::Path;

use alloy_primitives::Address;
use serde::Deserialize;

pub const CONFIG_PREFIX: &str = "PBH_CTF";

#[derive(Debug, Clone, Deserialize)]
pub struct CTFConfig {
    /// Semaphore secret
    pub semaphore_secret: String,
    /// WC Sepolia Provider
    pub provider_uri: String,
    /// Address of the player to keep score on the leaderboard
    pub player_address: Address,
}

impl CTFConfig {
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
