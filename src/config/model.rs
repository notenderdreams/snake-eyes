use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub watch: WatchConfig,
}

#[derive(Debug, Deserialize)]
pub struct WatchConfig {
    pub recursive: Option<bool>,
    pub debounce: Option<u64>,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            recursive: Some(true),
            debounce: Some(500),
        }
    }
}

impl WatchConfig {
    pub fn merge(&mut self, other: WatchConfig) {
        if let Some(recursive) = other.recursive {
            self.recursive = Some(recursive);
        }
        if let Some(debounce) = other.debounce {
            self.debounce = Some(debounce);
        }
    }
}

impl Config {
    pub fn merge(&mut self, other: Config) {
        self.watch.merge(other.watch);
    }
}
