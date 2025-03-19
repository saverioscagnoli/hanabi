use std::sync::Arc;
use sysinfo::System;
use tauri::WebviewWindow;
use tokio::sync::Mutex;

mod cpu;
mod mem;

#[tauri::command]
pub fn spawn_metrics_threads(window: WebviewWindow) {
    let window = Arc::new(window);
    let system = Arc::new(Mutex::new(System::new()));

    cpu::spawn_usage_meter(&window, &system);
    cpu::spawn_temp_meter(&window);

    mem::spawn_usage_meter(&window, &system);
}
