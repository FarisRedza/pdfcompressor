use std::process::Command;

pub fn compress(input_arg: String, output_arg: String, compress_arg: String) {
    println!("Input arg: {}", &input_arg);
    println!("Output arg: {}", &output_arg);
    println!("Compress arg: {}", &compress_arg);
    
    Command::new("gs")
        .arg("-sDEVICE=pdfwrite")
        .arg("-dCompatibilityLevel=1.4")
        .arg( "-dNOPAUSE")
        .arg("-dQUIET")
        .arg( "-dBATCH")
        .arg(compress_arg.as_str())
        .arg(output_arg.as_str())
        .arg(input_arg.as_str())
        .spawn()
        .expect("ghostscript failed to execute");
}