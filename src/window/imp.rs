use crate::compress;

use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, gio};
use glib::clone;
use glib::subclass::InitializingObject;

// Object holding the state
#[derive(gtk::CompositeTemplate, Default)]
#[template(resource = "/org/pdfcompressor/resources/window.ui")]
pub struct Window {
    #[template_child]
    pub input_file_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub input_file_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub quality_dropdown: TemplateChild<gtk::DropDown>,

    #[template_child]
    pub output_file_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub output_file_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub compress_button: TemplateChild<gtk::Button>,
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
        klass.install_action_async(
            "window.open",
            None,
            |window, _action_name, _action_target| async move {
                let file_dialog = gtk::FileDialog::builder()
                    .title("Open File")
                    .accept_label("Open")
                    .modal(true)
                    .build();

                file_dialog.open(Some(&window), gio::Cancellable::NONE, move |file| {
                    if let Ok(file) = file {
                        let filename = file.path().expect("Couldn't get file path");
                        println!("{}", filename.display());
                    }
                })
            },
        );
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

        let quality = Rc::new(RefCell::new(""));
        self.quality_dropdown.connect_selected_item_notify(clone!(@strong quality => move |quality_dropdown| {
            let level = quality_dropdown.selected();
            let selected_level = match level {
                0 => "screen",
                1 => "ebook",
                2 => "printer",
                _ => panic!("Error getting quality level"),
            };
            quality.replace(selected_level);
        }));

        self.output_file_button.connect_clicked(move |_| {
            // let input_file = window.
        });

        self.compress_button.connect_clicked(move |_| {
            let compression = compress::CompressObject::new(
                String::from("/home/faris/Downloads/Report.pdf"),
                String::from("-sOutputFile=/home/faris/Downloads/Report_compressed.pdf"),
                String::from("-dPDFSETTINGS=/".to_owned() + quality.borrow().clone())
            );
            compress::print_args(&compression);
            // compression.compress_file();
        });
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}