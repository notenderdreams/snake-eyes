use crate::config::loader::load_config;
use crate::watcher::runner::watch;

pub fn run() -> i32 {
    let config = load_config();

    match watch(config) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}
