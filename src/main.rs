use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result, prelude::*}
};

#[derive(Parser)]
struct Cli {
    query: std::path::PathBuf,
    index: std::path::PathBuf,
    output: std::path::PathBuf,
    #[clap(default_value_t=false,short, long)]
    case_sensitive: bool,
    #[clap(default_value_t=0, short, long)]
    verbosity: usize,
    /*#[clap(default_value_t=false, short, long)]
    overwrite: bool*/ //got rid of bc apparently create_new isn't suppsed to be used in release
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let query = File::open(args.query).expect("Query must be a file");
    let index = File::open(args.index)?;
    
    let readerQ = BufReader::new(query);
    let mut readerI = BufReader::new(index);
    let mut searchable = String::new();
    readerI.read_to_string(&mut searchable)?; 
    if !args.case_sensitive {
        searchable = searchable.to_lowercase();
    }

    let mut file_text = String::new();
    for line in readerQ.lines() {
        let txt = if args.case_sensitive {line?.clone()} else {line?.to_lowercase()};
        let split_at = txt.find(" ").unwrap();
        let result = txt.split_at(split_at + 1);
        if searchable.contains(result.1) {
            if args.verbosity > 1 {println!("Contains card {}", result.1)};
            file_text.push_str("[x] ");
        } else {
            file_text.push_str("[ ] ");
        }
        file_text = file_text + result.1 + "\n";
    }
    let mut file = File::create(args.output)?;
    file.write_all(file_text.as_bytes())?;
    Ok(())
}