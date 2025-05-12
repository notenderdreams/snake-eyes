use std::fs;
use std::path::Path;
use toml;
use crate::config::model::Config;


pub fn load_config()-> Config{
    let config_file = Path::new(".sny");

    if let Ok(contents) = fs::read_to_string(config_file){
        toml::from_str::<Config>(&contents).unwrap_or_default()
    }else{
        Config::default()
    }
}