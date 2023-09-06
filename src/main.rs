use std::fs::File;
use std::io::prelude::*;

use colored::Colorize;
use structopt::StructOpt;

use honey::{lexer, parser};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Honey",
    about = "A blazingly fast failure of a programming language."
)]
struct Opt {
    /// The input file to compile
    #[structopt()]
    file: String,

    /// Prints a representation of all the intermediary steps
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() {
    let opt = Opt::from_args();

    let mut file = match File::open(opt.file) {
        Ok(file) => file,
        Err(e) => {
            println!("{} {}", "Failed to open given file.".red(), e);
            std::process::exit(2);
        }
    };

    let mut source = String::new();

    if let Some(e) = file.read_to_string(&mut source).err() {
        println!("{} {}", "Failed to read given file.".red(), e);
        std::process::exit(2);
    }

    let tokens = lexer::lex(&source);
    let program = parser::parse(&tokens);

    if opt.verbose {
        println!(
            "\n{}\n\n{}\n------------\n",
            "File contents:".bold().yellow(),
            source,
        );

        println!(
            "{} {:#?}\n\n------------\n",
            "Tokens:".bold().green(),
            tokens
        );

        println!("{} {:#?}\n", "Parsed program:".bold().blue(), program);
    }
}
