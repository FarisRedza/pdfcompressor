mod window;

use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, gio, glib, Application, CssProvider, StyleContext};
use window::Window;

const APP_ID: &str = "com.pdfcompressor.github";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("pdfcompressor.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("resources/ui.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}