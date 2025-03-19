// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    path::{Path, PathBuf},
};
use tokio::{fs, io};
use traccia::{error, info, Colorize, LogLevel};

struct CustomFormatter;

impl traccia::Formatter for CustomFormatter {
    fn format(&self, record: &traccia::Record) -> String {
        format!(
            "{} [{}]: {}",
            chrono::Local::now()
                .format("%Y/%m/%d %H:%M:%S")
                .to_string()
                .color(traccia::Color::BrightCyan),
            record.level.default_coloring(),
            record.message
        )
    }
}

async fn get_log_dir() -> Result<PathBuf, io::Error> {
    let state_dir = env::var("XDG_STATE_HOME")
        .unwrap_or_else(|_| format!("{}/.local/state", env::var("HOME").unwrap()));
    let state_dir = Path::new(&state_dir);

    let log_dir = state_dir.join("hanabi");

    if !fs::try_exists(&log_dir).await.unwrap_or(false) {
        fs::create_dir_all(&log_dir).await?
    }

    Ok(log_dir)
}

#[tokio::main]
async fn main() {
    if let Ok(log_dir) = get_log_dir().await {
        let name = format!(
            "{}.log",
            chrono::Local::now().format("%Y-%m-%d").to_string()
        );

        let log_file = log_dir.join(name);

        traccia::init_with_config(traccia::Config {
            level: LogLevel::Info,
            targets: vec![
                Box::new(traccia::Console::new()),
                Box::new(traccia::File::new(log_file, traccia::FileMode::Truncate).unwrap()),
            ],
            format: Some(Box::new(CustomFormatter)),
        });

        info!("Log files located at {}", log_dir.to_string_lossy());
    } else {
        traccia::init_default();

        error!("Failed to locate logs directory. Reverting to console only.");
    }

    hanabi_lib::run()
}
