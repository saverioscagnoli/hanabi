use std::{sync::Arc, time::Duration};
use sysinfo::{Components, System};
use tauri::{Emitter, WebviewWindow};
use tokio::sync::Mutex;
use traccia::error;

pub fn spawn_usage_meter(window: &Arc<WebviewWindow>, system: &Arc<Mutex<System>>) {
    tokio::spawn({
        let window = window.clone();
        let system = system.clone();
        // Todo: Parse config file for duration
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        async move {
            loop {
                let mut system = system.lock().await;

                system.refresh_cpu_usage();

                let usage = system.global_cpu_usage();

                if let Err(err) = window.emit("cpu-usage", usage) {
                    error!("There was an error during ipc: {}", err);
                }

                interval.tick().await;
            }
        }
    });
}

pub fn spawn_temp_meter(window: &Arc<WebviewWindow>) {
    tokio::spawn({
        let window = window.clone();
        // Todo: Parse config file for duration
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        let mut temps = Vec::<f32>::new();

        async move {
            loop {
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
