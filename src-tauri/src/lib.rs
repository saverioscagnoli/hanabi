use std::time::Duration;

use gtk::{
    gdk::{traits::MonitorExt, Display, WindowTypeHint},
    traits::{ContainerExt, CssProviderExt, GtkWindowExt, WidgetExt},
};
use gtk_layer_shell::LayerShell;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![exec])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.hide().unwrap();

            let gtk_window = gtk::ApplicationWindow::new(
                &main_window.gtk_window().unwrap().application().unwrap(),
            );

            let win_clone = main_window.clone();

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

                    println!("{avg} C");
                    _ = win_clone.emit("cpu-usage", cpu_usage);
                    std::thread::sleep(Duration::from_secs(1))
                }
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
