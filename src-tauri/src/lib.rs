use std::time::Duration;

use gtk::traits::{ContainerExt, GtkWindowExt, WidgetExt};
use hyprland::{
    data::{Client, Workspaces},
    event_listener::EventListener,
    shared::{HyprData, HyprDataActiveOptional},
};
use setup::InitDock;
use sysinfo::{Components, System};
use tauri::{Emitter, Manager};

mod metrics;
mod setup;

#[tauri::command]
fn exec(script: String) {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .spawn()
        .expect("Failed to execute command");
}

#[tauri::command]
fn fetch_workspaces() -> Vec<i32> {
    Workspaces::get()
        .expect("Failed to get workspaces")
        .iter()
        .map(|w| w.id)
        .collect::<Vec<i32>>()
}

#[tauri::command]
fn fetch_active_window_title() -> Option<String> {
    if let Ok(client) = Client::get_active() {
        if let Some(window) = client {
            return Some(window.title);
        }
    }

    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            exec,
            fetch_workspaces,
            fetch_active_window_title,
            metrics::spawn_metrics_threads
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.hide().unwrap();

            let gtk_window =
                gtk::ApplicationWindow::new(&window.gtk_window().unwrap().application().unwrap());

            let win_clone_wm = window.clone();

            std::thread::spawn(move || {
                let mut event_listener = EventListener::new();

                event_listener.add_active_window_changed_handler({
                    let win_clone_wm = win_clone_wm.clone();
                    move |data| {
                        if let Some(data) = data {
                            let _ = win_clone_wm.emit("window-changed", data.title);
                        }
                    }
                });

                event_listener.add_workspace_changed_handler({
                    let win_clone_wm = win_clone_wm.clone();
                    move |data| {
                        let w = fetch_workspaces();
                        let _ = win_clone_wm.emit("workspace-changed", (data.id, w));
                    }
                });

                event_listener
                    .start_listener()
                    .expect("Failed to start hyprland listener");
            });

            if let Ok(vbox) = window.default_vbox() {
                // Remove default box from the webview window.
                if let Ok(window) = window.gtk_window() {
                    window.remove(&vbox);
                }

                // Add the box to the newly created gtk window
                gtk_window.add(&vbox);
            }

            gtk_window.init_dock();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
