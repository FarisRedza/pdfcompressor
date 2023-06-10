use glib::subclass::InitializingObject;
use gtk::{prelude::*, ApplicationWindow};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate,
        FileChooserDialog, FileChooserAction, ResponseType};

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/pdfcompressor/resources/window.ui")]
pub struct Window {
    pub window: ApplicationWindow,
    #[template_child]
    pub file_chooser_button: TemplateChild<Button>,
    #[template_child]
    pub compress_button: TemplateChild<Button>,
}
// ANCHOR_END: object

// ANCHOR: subclass
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
// ANCHOR_END: subclass

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        // let app_window = gtk::ApplicationWindow;

        // self.file_chooser_button.connect_clicked(move |file_chooser_button| {
        //     let file_dialog = FileChooserDialog::new(
        //         Some("Select File"),
        //         Some(app_window),
        //         FileChooserAction::Open,
        //         &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        //     );
        // });

        // Connect to "clicked" signal of `button`
        self.compress_button.connect_clicked(move |compress_button| {
            // Set the label to "Hello World!" after the button has been clicked on
            compress_button.set_label("Compressing!");
            
            let input_arg = String::from("/home/faris/Downloads/less.pdf");
            let output_arg = String::from("-sOutputFile=/home/faris/Downloads/less_compressed.pdf");
            let compress_arg = String::from("-dPDFSETTINGS=/screen");
            
            pdfcompressor::compress(input_arg, output_arg, compress_arg);
        });
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}