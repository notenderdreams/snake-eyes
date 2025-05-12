use notify::{
    RecommendedWatcher,
    RecursiveMode,
    Watcher
};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::{
    Duration,
    Instant 
};



pub fn watch(recursive: bool) -> notify::Result<()> {
    let watch_dir = PathBuf::from(".");
    let debounce_duration = Duration::from_millis(1000);
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        notify::recommended_watcher(move |res| {
            if let Ok(_event) = res {
                let _ = tx.send(());
            }
        })?;

    let mode = if recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    watcher.watch(&watch_dir, mode)?;

    let mut last_event = Instant::now() - debounce_duration;

    println!("(recursive: {})", recursive);

    loop {
        if let Ok(_) = rx.recv() {
            let now = Instant::now();
            if now.duration_since(last_event) >= debounce_duration {
                println!("Change detected at {:?}", now);
                last_event = now;
            }
        }
    }
}