mod imp;

// use gio::File;
use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app)
            .build()
    }

    // fn set_pdf_file(&self, file: File) {
    //     let file_path = self.imp().file.to_string();
    //     println!("{}", file_path);
    // }
}