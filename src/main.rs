use gtk::{
    glib, prelude::*,
    Application, ApplicationWindow, Box, Orientation,
    Button, Label, MenuButton,
    FileChooserAction, FileChooserDialog, ResponseType
};

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
    let intro_label = Label::builder()
        .label("Select a PDF file to compress")
        .margin_top(12)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();


    let file_button = Button::builder()
        .label("Select File")
        .margin_top(12)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();

    let file_label = Label::builder()
        .label("No file selected")
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();

        let file_size_label = Label::builder()
        .label("File size:")
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Create the quality menu button
    let quality_menubutton = MenuButton::builder()
        .label("Quality Level")
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();

        let compress_button = Button::builder()
        .label("Compress")
        .margin_top(6)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&intro_label);
    content.append(&file_button);
    content.append(&file_label);
    content.append(&file_size_label);
    content.append(&quality_menubutton);
    content.append(&compress_button);

    let window = ApplicationWindow::builder()
        .title("PDF Compressor")
        .application(app)
        .default_width(320)
        .default_height(200)
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

                pdfcompressor::compress(input_arg, output_arg, compress_arg);
            }
            d.close();
        });
    
        file_chooser.show();
    }));

    window.show();
}