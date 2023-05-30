#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::path::PathBuf;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("pdfcompressor")
        .version("0.1.0")
        .author("Faris Redza <faris.redza1999@gmail.com")
        .about("A PDF file compressor frontend for ghostscript")
        .arg(Arg::new("low quality")
                .short('l')
                .long("low")
                .help("Output files will have up to 72 DPI"))
        .arg(Arg::new("medium quality")
                .short('m')
                .long("medium")
                .help("Output files will have up to 150 DPI"))
        .arg(Arg::new("high quality")
                .short('H')
                .long("high")
                .help("Output files will have up to 300 DPI"))
        .get_matches();

    let (input_arg, output_arg, compress_arg) = pdfcompressor::parse_input();
    pdfcompressor::compress(input_arg, output_arg, compress_arg);
}