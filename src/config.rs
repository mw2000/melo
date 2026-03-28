use std::path::PathBuf;

use serde::Deserialize;

/// User configuration loaded from `~/.config/melo/config.toml`.
/// All fields are optional — missing values fall back to defaults.
/// CLI flags take precedence over config file values.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub theme: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        Self::config_path()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn config_path() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(|home| {
            PathBuf::from(home)
                .join(".config")
                .join("melo")
                .join("config.toml")
        })
    }
}
