use sysinfo::System;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::Mutex;
use std::{sync::Arc, time::Duration};
use traccia::error;

pub fn spawn_usage_meter(window: &Arc<WebviewWindow>, system: &Arc<Mutex<System>>) {
    tokio::spawn({
        let window = window.clone();
        let system = system.clone();

        let mut interval = tokio::time::interval(Duration::from_secs(5));

        async move {
            loop {
                let mut system = system.lock().await;

                system.refresh_memory();

                let usage = system.used_memory() as f64;
                let total = system.total_memory() as f64;

                let percent = usage / total * 100.0;

                if let Err(err) = window.emit("mem-usage", percent) {
                    error!("There was an error during ipc: {}", err);
                }

                interval.tick().await;
            }
        }
    });
}