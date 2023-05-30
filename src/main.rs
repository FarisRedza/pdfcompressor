#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::process::Command;
use std::path::PathBuf;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() {
    let (input_arg, output_arg, compress_arg) = parse_input();
    compress(input_arg, output_arg, compress_arg);
}

fn parse_input() -> (String, String, String) {
    let args = Cli::parse();
    let output_file = args.path.clone().with_extension("compressed.pdf");
    
    let compress_arg = format!("-dPDFSETTINGS=/{}", args.pattern);
    let input_arg = String::from(args.path.into_os_string().into_string().unwrap());
    let output_arg = format!("-sOutputFile={}", output_file.into_os_string().into_string().unwrap());

    return (input_arg, output_arg, compress_arg);
}

fn compress(input_arg: String, output_arg: String, compress_arg: String) {
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