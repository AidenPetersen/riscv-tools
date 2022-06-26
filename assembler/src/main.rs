mod assembler;
mod parser;
mod instructions;

use clap::Parser;
use std::fs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser=file_exists, help="Input assembly file" )]
    input_file: String,
    
    #[clap(value_parser, help="Output ELF binary executable")]
    output_file: String,
}

fn main() {
    let cli = Cli::parse();
    
    // Read file
    let contents: String = fs::read_to_string(cli.input_file).unwrap();
    print!("{}", contents);
}

fn file_exists(s: &str) -> Result<String, String> {
    let input: String= String::from(s);
    if std::path::Path::new(&s).exists() {
        Ok(input)
    } else {
        Err(format!("file `{}` does not exist", s))
    }

}
