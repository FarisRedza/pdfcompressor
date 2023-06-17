use std::fmt::{Display, Formatter, Result};
use std::process::Command;

#[derive(Debug, Default, Clone)]
pub struct CompressObject {
    pub input_arg: String,
    pub output_arg: String,
    pub quality_arg: String,
}

impl CompressObject {
    pub fn new(input_arg: String, output_arg: String, quality_arg: String) -> CompressObject {
        CompressObject {
            input_arg,
            output_arg,
            quality_arg,
        }
    }

    pub fn compress_file(&self) {
        dbg!(&self);
        Command::new("gs")
            .arg("-sDEVICE=pdfwrite")
            .arg("-dCompatibilityLevel=1.4")
            .arg( "-dNOPAUSE")
            .arg("-dQUIET")
            .arg( "-dBATCH")
            .arg(self.quality_arg.as_str())
            .arg(self.output_arg.as_str())
            .arg(self.input_arg.as_str())
            .status()
            .expect("ghostscript failed to execute");
    }
}

impl Display for CompressObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Input: {}\nOutput: {}\nCompression: {}",
            self.input_arg, self.output_arg, self.quality_arg)
    }
}

pub fn print_args<T>(args: T)
where 
    T: Display,
{
    println!("CompressObject arguments:\n{}", args);
}