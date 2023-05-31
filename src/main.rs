use clap::Parser;

/// Simple frontend for ghostscript to compress PDF files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Compression level (low, medium, high)
    #[arg(short, long, default_value_t = String::from("medium"))]
    compression: String,

    /// Input file name
    #[arg(short, long)]
    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let compress_arg = match args.compression.as_ref() {
        "low" => String::from("-dPDFSETTINGS=/printer"),
        "medium" => String::from("-dPDFSETTINGS=/ebook"),
        "high" => String::from("-dPDFSETTINGS=/screen"),
        _ => panic!("Error: Invalid compression level"),
    };
    
    let file_check = args.file.clone().exists();
    if file_check == true {
        let output_file = args.file.clone().with_extension("compressed.pdf");
    
        let input_arg = String::from(args.file.into_os_string().into_string().unwrap());
        let output_arg = format!("-sOutputFile={}", output_file.into_os_string().into_string().unwrap());

        pdfcompressor::compress(input_arg, compress_arg, output_arg);
    } else {
        panic!("Error: No file found");
    }
}