use std::process::Command;

#[derive(Debug)]
pub struct Arguments {
    pub input: String,
    pub output: String,
    pub compress: String,
}

pub fn compress(args: &Arguments) {
    dbg!(&args);
    Command::new("gs")
        .arg("-sDEVICE=pdfwrite")
        .arg("-dCompatibilityLevel=1.4")
        .arg( "-dNOPAUSE")
        .arg("-dQUIET")
        .arg( "-dBATCH")
        .arg(args.compress.as_str())
        .arg(args.output.as_str())
        .arg(args.input.as_str())
        .status()
        .expect("ghostscript failed to execute");
}