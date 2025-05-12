use crate::config::model::Config;
use std::fs;
use std::path::Path;
use toml;

pub fn load_config() -> Config {
    let config_file = Path::new(".sny");

    let mut config = Config::default();

    if let Ok(contents) = fs::read_to_string(config_file) {
        match toml::from_str::<Config>(&contents) {
            Ok(parsed) => {
                config.merge(parsed);
            }
            Err(e) => {
                eprintln!("Failed to parse config: {}", e);
                return Config::default();
            }
        }
    }

    config
}
