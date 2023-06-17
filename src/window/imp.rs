use crate::compress;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
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

        let mut compression = compress::CompressObject{
            quality: String::from("-dPDFSETTINGS=/"),
            ..Default::default()
        };

        let number = Rc::new(Cell::new(0));
        self.input_file_button.connect_clicked(clone!(@weak number => move |_| {
            number.set(number.get() + 1);
        }));

        println!("{:?}", &number);

        let quality = Rc::new(RefCell::new("test"));
        self.quality_dropdown.connect_selected_item_notify(clone!(@strong quality => move |quality_dropdown| {
            let option = quality_dropdown.selected();
            let level = match option {
                0 => "screen",
                1 => "ebook",
                2 => "printer",
                _ => panic!("Error getting quality level"),
            };
            let test = &quality;
            println!("{:?}", &quality);
            // quality.set(option);
        }));

        // compression.quality.push_str(level);
        // println!("{:?}", &compression);

        self.output_file_button.connect_clicked(move |_| {
            println!("Hello there again");
        });

        self.compress_button.connect_clicked(move |_| {
            println!("Compressing");
        });
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}