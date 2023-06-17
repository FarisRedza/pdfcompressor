use std::process::Command;

#[derive(Debug, Default)]
pub struct CompressObject {
    pub input_arg: String,
    pub output_arg: String,
    pub quality: String,
}

impl CompressObject {
    pub fn compress_file(&self) {
        dbg!(&self);
        Command::new("gs")
            .arg("-sDEVICE=pdfwrite")
            .arg("-dCompatibilityLevel=1.4")
            .arg( "-dNOPAUSE")
            .arg("-dQUIET")
            .arg( "-dBATCH")
            .arg(self.quality.as_str())
            .arg(self.output_arg.as_str())
            .arg(self.input_arg.as_str())
            .status()
            .expect("ghostscript failed to execute");
    }
}