use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub watch: WatchConfig,
}

//Actual Config
#[derive(Debug, Deserialize)]
pub struct WatchConfig {
    pub recursive: bool,
    pub debounce: u64,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            recursive: true,
            debounce: 500,
        }
    }
}
