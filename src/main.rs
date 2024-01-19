use std::{io::{BufReader, BufRead}};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("Pattern {:?}, path: {:?}", args.pattern,args.path);
    
    let file = std::fs::File::open(&args.path).expect("File not found at that path");
    let mut buf_reader = BufReader::new(&file);
    let mut line = String::new();

    loop {
        let line_size = buf_reader.read_line(&mut line).expect("Error reading line");
        if line_size == 0 {
            break;
        }

        if line.contains(&args.pattern) {
            println!("{:?} -> Bytes: {:?}", line, line_size);
        }

        line.clear();
    }
}
