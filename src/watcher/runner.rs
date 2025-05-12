use crate::config::model::Config;
use crate::logger::{FileChangeType, Logger};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

pub fn watch(config: Config) -> notify::Result<()> {
    let watch_dir = PathBuf::from(".");

    let debounce_duration = Duration::from_millis(config.watch.debounce.unwrap());
    let recursive = config.watch.recursive.unwrap();

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.send(event);
        }
    })?;

    let mode = if recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    watcher.watch(&watch_dir, mode)?;

    let mut last_event = Instant::now() - debounce_duration;

    Logger::init();

    loop {
        if let Ok(event) = rx.recv() {
            let now = Instant::now();

            if now.duration_since(last_event) >= debounce_duration {
                last_event = now;

                for path in event.paths {
                    let file_path = path.display().to_string();

                    match event.kind {
                        EventKind::Create(_) => {
                            Logger::file_event(FileChangeType::Created, &file_path);
                        }
                        EventKind::Modify(_) => {
                            Logger::file_event(FileChangeType::Modified, &file_path);
                        }
                        EventKind::Remove(_) => {
                            Logger::file_event(FileChangeType::Deleted, &file_path);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
