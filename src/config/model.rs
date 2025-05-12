use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    pub watch : WatchConfig,
}


#[derive(Debug,Deserialize)]
pub struct WatchConfig{
    pub recursive: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch: WatchConfig::default(),
        }
    }
}


impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            recursive: true,
        }
    }
}