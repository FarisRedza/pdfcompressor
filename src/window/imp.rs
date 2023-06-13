use glib::subclass::InitializingObject;
use gtk::{prelude::*, ApplicationWindow};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, MenuButton, CompositeTemplate, FileDialog};
use pdfcompressor::Arguments;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/pdfcompressor/resources/window.ui")]
pub struct Window {
    #[template_child]
    pub file_chooser_button: TemplateChild<Button>,
    #[template_child]
    pub quality_menubutton: TemplateChild<Button>,
    #[template_child]
    pub compress_button: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "PDFCompressor";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Connect to "clicked" signal of `file_chooser_button`
        self.file_chooser_button.connect_clicked(move |file_chooser_button| {
            println!("Dank memes");
        });

        // Connect to "clicked" signal of `file_chooser_button`
        self.quality_menubutton.connect_clicked(move |quality_menubutton| {
            println!("Hello there");
        });
        
        // Connect to "clicked" signal of `compress_button`
        self.compress_button.connect_clicked(move |compress_button| {
            let args = Arguments{
                input: String::from("/home/faris/Downloads/Report.pdf"),
                output: String::from("-sOutputFile=/home/faris/Downloads/Report_compressed.pdf"),
                compress: String::from("-dPDFSETTINGS=/screen"),
            };
            pdfcompressor::compress(&args);
        });
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}