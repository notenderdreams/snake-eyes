use crate::logger::FileChangeType;
use chrono::Local;
use colorized::{Color, Colors};

pub struct Logger;

impl Logger {
    pub fn init() {
        Self::log(
            "Waiting for changes. Press Ctrl+C to stop.",
            "INIT",
            Colors::WhiteBg,
        );
    }

    pub fn info(msg: &str) {
        Self::log(msg, "INFO", Colors::BrightBlackFg);
    }

    pub fn file_event(file_change: FileChangeType, file_path: &str) {
        let (label, action, color) = match file_change {
            FileChangeType::Created => ("FILE+", "Created", Colors::CyanFg),
            FileChangeType::Modified => ("FILE~", "Modified", Colors::MagentaFg),
            FileChangeType::Deleted => ("FILE-", "Deleted", Colors::RedFg),
        };

        let message = format!("{} {}: {}", action, action, file_path);
        Self::log(&message, label, color);
    }

    pub fn success(msg: &str) {
        Self::log(msg, "SUCCESS", Colors::GreenFg);
    }

    pub fn debug(msg: &str) {
        Self::log(msg, "DEBUG", Colors::BrightYellowBg);
    }

    pub fn warning(msg: &str) {
        Self::log(msg, "WARNING", Colors::YellowFg);
    }

    pub fn error(msg: &str) {
        Self::log(msg, "ERROR", Colors::RedBg);
    }

    pub fn custom(msg: &str, label: &str, color: Colors) {
        Self::log(msg, label, color);
    }

    fn log(msg: &str, label: &str, color: Colors) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let timestamp_colored = timestamp.color(Colors::BlueFg);

        let padded_label = format!("{:<7}", label);
        let label_colored = padded_label.color(color);

        println!("[{}] [{}] {}", timestamp_colored, label_colored, msg);
    }
}
