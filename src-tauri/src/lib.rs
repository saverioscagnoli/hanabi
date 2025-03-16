use std::time::Duration;

use gtk::{
    gdk::{traits::MonitorExt, Display, WindowTypeHint},
    traits::{ContainerExt, CssProviderExt, GtkWindowExt, WidgetExt},
};
use gtk_layer_shell::LayerShell;
use hyprland::{
    data::{Workspace, Workspaces},
    event_listener::EventListener,
    shared::HyprData,
};
use sysinfo::{Components, System};
use tauri::{Emitter, Manager};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![exec, fetch_workspaces])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.hide().unwrap();

            let gtk_window = gtk::ApplicationWindow::new(
                &main_window.gtk_window().unwrap().application().unwrap(),
            );

            let win_clone = main_window.clone();
            let win_clone_wm = main_window.clone();

            std::thread::spawn(move || {
                let mut system = System::new_all();

                loop {
                    system.refresh_all();
                    let cpu_usage = system.global_cpu_usage();
                    let cpu = system.cpus().first().unwrap();

                    let components = Components::new_with_refreshed_list();
                    let mut temps = Vec::new();

                    for c in &components {
                        if c.label().starts_with("Core") {
                            if let Some(temp) = c.temperature() {
                                temps.push(temp);
                            }
                        }
                    }

                    let avg = temps.iter().sum::<f32>() / temps.len() as f32;

                    _ = win_clone.emit("cpu-usage", cpu_usage);
                    std::thread::sleep(Duration::from_secs(1))
                }
            });

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

            // To prevent the window from being black initially.
            gtk_window.set_app_paintable(true);

            let vbox = main_window.default_vbox().unwrap();
            main_window.gtk_window().unwrap().remove(&vbox);
            gtk_window.add(&vbox);

            // Doesn't throw errors.
            gtk_window.init_layer_shell();

            // Just works.
            gtk_window.set_layer(gtk_layer_shell::Layer::Top);

            let display = Display::default().expect("No default display found");
            let monitor = display.primary_monitor().unwrap_or_else(|| {
                // Fallback to the first monitor if primary isn't available
                display.monitor(0).expect("No monitors found")
            });
            let width = monitor.geometry().width();

            gtk_window.set_width_request(width);
            gtk_window.set_height_request(30);

            gtk_window.set_anchor(gtk_layer_shell::Edge::Top, true);
            gtk_window.set_anchor(gtk_layer_shell::Edge::Left, true);
            gtk_window.set_anchor(gtk_layer_shell::Edge::Right, true);

            gtk_window.set_keep_above(true);
            gtk_window.set_resizable(false);
            gtk_window.set_type_hint(WindowTypeHint::Dock);

            gtk_window.set_exclusive_zone(30);

            gtk_window.show_all();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
