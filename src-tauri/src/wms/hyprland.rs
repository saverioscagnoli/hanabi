use std::sync::Arc;

use hyprland::{
    data::{Client, Workspaces},
    event_listener::EventListener,
    shared::{HyprData, HyprDataActiveOptional},
};
use tauri::{Emitter, WebviewWindow};
use traccia::error;

pub fn fetch_workspaces() -> Vec<i32> {
    Workspaces::get()
        .expect("Failed to get workspaces")
        .iter()
        .map(|w| w.id)
        .collect::<Vec<i32>>()
}

pub fn fetch_active_window_title() -> Option<String> {
    if let Ok(client) = Client::get_active() {
        if let Some(window) = client {
            return Some(window.title);
        }
    }

    None
}

pub fn spawn_listeners(window: WebviewWindow) {
    let window = Arc::new(window);

    tokio::spawn(async move {
        let mut event_listener = EventListener::new();

        event_listener.add_active_window_changed_handler({
            let window = window.clone();
            move |data| {
                if let Some(data) = data {
                    if let Err(err) = window.emit("window-changed", data.title) {
                        error!("There was an error during ipc: {}", err);
                    }
                }
            }
        });

        event_listener.add_workspace_changed_handler({
            let window = window.clone();
            move |data| {
                let w = fetch_workspaces();
                if let Err(err) = window.emit("workspace-changed", (data.id, w)) {
                    error!("There was an error during ipc: {}", err);
                }
            }
        });

        if let Err(err) = event_listener.start_listener() {
            error!("Failed to start hyprland event listener: {}", err);
        }
    });
}
