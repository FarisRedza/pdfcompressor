use std::process::Command;

pub fn compress(input_arg: String, output_arg: String, compress_arg: String) {
    Command::new("gs")
        .args([
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            "-dNOPAUSE",
            "-dQUIET",
            "-dBATCH",
            compress_arg.as_str(),
            output_arg.as_str(),
            input_arg.as_str(),
        ])
        .spawn()
        .expect("ghostscript failed to execute");
}