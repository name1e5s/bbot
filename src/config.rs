use once_cell::sync::Lazy;
use serde::Deserialize;

use anyhow::Result;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    fn inner() -> Result<Config> {
        let config = std::fs::read_to_string("config.toml")?;
        toml::from_str(&config).map_err(Into::into)
    }
    inner().unwrap()
});

#[derive(Deserialize)]
pub struct Config {
    pub site: Site,
    pub transmission: Transmission,
}

#[derive(Deserialize)]
pub struct Site {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Transmission {
    pub url: String,
    pub dir: String,
    pub user: String,
    pub password: String,
}

pub fn config() -> &'static Config {
    &CONFIG
}
