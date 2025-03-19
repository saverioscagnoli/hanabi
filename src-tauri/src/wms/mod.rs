pub mod hyprland;

use std::{env, sync::LazyLock};
use traccia::error;

static COMPOSITOR: LazyLock<Compositor> = LazyLock::new(|| Compositor::detect());

#[derive(Debug, Clone, Copy)]
pub enum Compositor {
    Hyprland,
    Sway,
    Unknown,
}

impl Compositor {
    fn detect() -> Self {
        if let Ok(wayland_display) = env::var("WAYLAND_DISPLAY") {
            println!("Wayland is running, display: {}", wayland_display);

            if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
                match desktop.to_lowercase().as_str() {
                    "hyprland" => Self::Hyprland,
                    "sway" => Self::Sway,
                    _ => Self::Unknown,
                }
            } else {
                todo!("Fallback");
            }
        } else {
            error!("Currently only wayland is supported!");
            Self::Unknown
        }
    }

    pub fn current() -> Self {
        *COMPOSITOR
    }
}
