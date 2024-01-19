use std::io::{BufReader, BufRead};
use clap::Parser;
use std::fmt::Debug;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    println!("Pattern {:?}, path: {:?}", args.pattern,args.path);
    
    let file = std::fs::File::open(&args.path)
        .map_err(|err| CustomError(format!("Error Reading: `{:?}`: {}", args.path, err)))?;

    // .expect() above is a shortcut to a block similar to below
    // let file = match file_open_result {
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Can't deal with this file opening error: {}", error)}
    // };

    let mut buf_reader = BufReader::new(&file);
    let mut line = String::new();

    // Due to custom return type, need to wrap in Ok()
    Ok(loop  {
        let line_size = buf_reader.read_line(&mut line).expect("Error reading line");
        if line_size == 0 {
            break;
        }

        if line.contains(&args.pattern) {
            println!("{:?} -> Bytes: {:?}", line, line_size);
        }

        line.clear();
    })
}
