use gtk::traits::{ContainerExt, GtkWindowExt};
use setup::InitDock;
use std::env;
use tauri::{Manager, WebviewWindow};
use wms::Compositor;

mod metrics;
mod setup;
mod wms;

#[tauri::command]
fn exec(script: String) {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .spawn()
        .expect("Failed to execute command");
}

#[tauri::command]
fn init_compositor_events(window: WebviewWindow) {
    match Compositor::current() {
        Compositor::Hyprland => wms::hyprland::spawn_listeners(window),
        Compositor::Sway => todo!("Implement sway support"),
        _ => {}
    }
}

#[tauri::command]
fn fetch_workspaces() -> Vec<i32> {
    match Compositor::current() {
        Compositor::Hyprland => wms::hyprland::fetch_workspaces(),
        Compositor::Sway => todo!("Implement sway support"),
        _ => Vec::new(),
    }
}

#[tauri::command]
fn fetch_active_window_title() -> Option<String> {
    match Compositor::current() {
        Compositor::Hyprland => wms::hyprland::fetch_active_window_title(),
        _ => None,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            exec,
            init_compositor_events,
            fetch_workspaces,
            fetch_active_window_title,
            metrics::spawn_metrics_threads
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.hide().unwrap();

            let gtk_window =
                gtk::ApplicationWindow::new(&window.gtk_window().unwrap().application().unwrap());

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
