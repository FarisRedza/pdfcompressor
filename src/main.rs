mod window;
mod application;
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
    app.connect_startup(|_| application::App::load_css());
    app.connect_activate(application::App::build_ui);

    // Run the application
    app.run()
}