use gtk::{
    gdk::{traits::MonitorExt, Display, WindowTypeHint},
    traits::{GtkWindowExt, WidgetExt},
    ApplicationWindow,
};
use gtk_layer_shell::{Edge, Layer, LayerShell};

pub trait InitDock {
    fn init_dock(&self);
}

impl InitDock for ApplicationWindow {
    fn init_dock(&self) {
        // Crucial for window transparency
        self.set_app_paintable(true);

        // Initialize the layer shell protocol
        self.init_layer_shell();

        // Set the window to be top level
        self.set_layer(Layer::Top);

        if let Some(display) = Display::default() {
            let monitor = display
                .primary_monitor()
                .unwrap_or_else(|| display.monitor(0).expect("No monitors found."));

            let width = monitor.geometry().width();

            self.set_width_request(width);
            // TODO: parse height from config file
            self.set_height_request(30);
            self.set_exclusive_zone(30);

            self.set_anchor(Edge::Top, true);
            self.set_anchor(Edge::Left, true);
            self.set_anchor(Edge::Right, true);

            // Set window parameters
            self.set_keep_above(true);
            self.set_resizable(false);

            // Dock the window
            self.set_type_hint(WindowTypeHint::Dock);

            // Show the gtk window
            self.show_all();
        }
    }
}
