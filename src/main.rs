use clap::Parser;
use std::path::PathBuf;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation};

/// Simple frontend for ghostscript to compress PDF files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Compressed output file quality (low, medium, high)
    #[arg(short, long, default_value_t = String::from("medium"))]
    quality: String,

    /// Input file name
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let app = Application::builder()
        .application_id("com.jwestall.ui-demo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .label("Press to compress")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = Button::builder()
        .label("Compress File")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("PDF Compressor")
        .application(app)
        .child(&content)
        .build();

    button.connect_clicked(move |_| compress_button(&label));

    window.show();
}

fn compress_button(label: &Label) {
    label.set_text("Compressing");
}