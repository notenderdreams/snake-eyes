use notify::{Watcher, RecursiveMode,RecommendedWatcher};
use std::{
    path::PathBuf,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

fn main() {
    let watch_dir = PathBuf::from(".");           
    let debounce_duration = Duration::from_millis(1000); 
    let (tx, rx) = channel();                       

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        if let Ok(_event) = res {
            let _ = tx.send(()); 
        }
    }).unwrap();

    watcher.watch(&watch_dir, RecursiveMode::Recursive).unwrap(); 

    let mut last_event = Instant::now() - debounce_duration; 

    loop {
        if let Ok(_) = rx.recv() {
            let now = Instant::now(); 
            if now.duration_since(last_event) >= debounce_duration {
                println!("{:?}", now); 
                last_event = now;        
            }
        }
    }
}
