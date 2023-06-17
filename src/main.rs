mod window;
mod compress;

// use glib::{MainContext, Priority};
use gtk::prelude::*;
use gtk::{glib, gdk, gio};

const APP_ID: &str = "com.pdfcompressor.github";

pub enum Event {
    SelectFile,
    CompressFile,
    Closed,
    Quit,
}

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("pdfcompressor.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &gtk::Application) {
    // let (tx, rx) = glib::MainContext::channel(glib::Priority::default());

    // Create new window and present it
    let window = window::Window::new(app);
    window.present();
    
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("resources/ui.css"));

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}