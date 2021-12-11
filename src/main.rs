use std::path::Path;
use anyhow::{Result, bail};
use clap::{App, Arg};

mod lexer;

fn main() -> Result<()> {
    // Parse command line args
    let args = App::new("winter")
        .version("0.1.0")
        .about("Winter(preter) can run W code. W is a simple language, this will just do the basic operations as described in the documentation!")
        .author("ArcticSpaceFox")
        .args(&[
            Arg::with_name("file")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true)
            .value_name("FILE_TO_INTERPRET")
            .help("The file to interpret"),
        ]).get_matches();
    
    // Get input file
    let file_path = Path::new(args.value_of("file").expect("No file path"));
    if !file_path.exists() {
        bail!("This file does not exist");
    }

    let input = std::fs::read_to_string(file_path).expect("Could not read file");
    let l = lexer::Lexer::new(&input);
    for t in l {
        println!("{:?}", t);
    }

    Ok(())
}
