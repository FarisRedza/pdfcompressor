use gtk::prelude::*;
use gtk::{glib, gdk};
use crate::window;

pub struct App {
}

impl App {
    pub fn build_ui(app: &gtk::Application) {
        // Create new window and present it
        let window = window::Window::new(app);
        window.present();
    }

    pub fn load_css() {
        // Load the CSS file and add it to the provider
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("resources/ui.css"));

        // Add the provider to the default screen
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}