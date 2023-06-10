use gtk::glib;
use gtk::prelude::*;

use gtk::{
    Application, ApplicationWindow, Box, Orientation, Button,
    FileChooserAction, FileChooserDialog, ResponseType
};

use std::process::Command;

const APP_ID: &str = "com.github.pdfcompressor";

fn main() {
    let app = Application::new(
        Some(APP_ID),
        Default::default(),
    );
    app.connect_activate(build_ui);
    app.run();
}

pub fn build_ui(app: &Application) {
    let file_button = Button::builder()
    .label("Select File")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&file_button);

    let window = ApplicationWindow::builder()
        .title("PDF Compressor")
        .application(app)
        .child(&content)
        .build();

    file_button.connect_clicked(glib::clone!(@weak window => move |_| {
        let file_chooser = FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            FileChooserAction::Open,
            &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );

        file_chooser.connect_response(move |d: &FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let filename = file.path().expect("Couldn't get file path");

                let input_arg = filename.clone().into_os_string().into_string().unwrap();
                
                let output_file = filename.with_extension("").into_os_string().into_string().expect("Error converting file path to string");
                let output_arg = format!("-sOutputFile={}_compressed.pdf", output_file);

                let compress_arg = String::from("-dPDFSETTINGS=/screen");

                println!("Input arg: {}", &input_arg);
                println!("Output arg: {}", &output_arg);
                println!("Compress arg: {}", &compress_arg);

                compress(input_arg, output_arg, compress_arg);
            }
            d.close();
        });
    
        file_chooser.show();
    }));

    window.show();
}

pub fn compress(input_arg: String, output_arg: String, compress_arg: String) {
    Command::new("gs")
        .arg("-sDEVICE=pdfwrite")
        .arg("-dCompatibilityLevel=1.4")
        .arg( "-dNOPAUSE")
        .arg("-dQUIET")
        .arg( "-dBATCH")
        .arg(compress_arg.as_str())
        .arg(output_arg.as_str())
        .arg(input_arg.as_str())
        .spawn()
        .expect("ghostscript failed to execute");
}