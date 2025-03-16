use std::{sync::Arc, time::Duration};
use sysinfo::{Components, System};
use tauri::{Emitter, WebviewWindow};
use tokio::sync::Mutex;
use traccia::error;

mod cpu;

#[tauri::command]
pub fn spawn_metrics_threads(window: WebviewWindow) {
    let system = Arc::new(Mutex::new(System::new()));

    tokio::spawn({
        let window = window.clone();
        let system = system.clone();
        // Todo: Parse config file for duration
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let mut temps = Vec::<f32>::new();

        async move {
            loop {
                let mut system = system.lock().await;

                system.refresh_cpu_all();

                let components = Components::new_with_refreshed_list();

                for c in &components {
                    if c.label().starts_with("Core") {
                        if let Some(t) = c.temperature() {
                            temps.push(t);
                        }
                    }
                }

                let average = temps.iter().sum::<f32>() / temps.len() as f32;

                temps.clear();

                // Emit to frontend
                if let Err(err) = window.emit("cpu-temp", average) {
                    error!("There was an error during ipc: {}", err);
                }

                interval.tick().await;
            }
        }
    });
}
