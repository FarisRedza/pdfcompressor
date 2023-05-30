#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::path::PathBuf;


fn main() {
    let (input_arg, output_arg, compress_arg) = pdfcompressor::parse_input();
    pdfcompressor::compress(input_arg, output_arg, compress_arg);
}