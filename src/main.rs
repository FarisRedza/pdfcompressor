use clap::Parser;
use std::{path::PathBuf};

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
    let args = Args::parse();
    
    let compress_arg = match args.quality.as_ref() {
        "low" => String::from("-dPDFSETTINGS=/screen"),
        "medium" => String::from("-dPDFSETTINGS=/ebook"),
        "high" => String::from("-dPDFSETTINGS=/printer"),
        _ => panic!("Error: Invalid output file quality, use low, medium, or high"),
    };
    
    let input_arg = match args.file.with_extension("").into_os_string().into_string() {
        Err(why) => panic!("Failed to read file: {:?}", why),
        Ok(input_arg) => format!("{}.pdf", input_arg),
    };

    let output_arg = format!("-sOutputFile={}_compressed.pdf", &input_arg);
    
    pdfcompressor::compress(input_arg, compress_arg, output_arg);
}