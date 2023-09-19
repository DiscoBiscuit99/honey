use std::fs::File;
use std::io::prelude::*;

use colored::Colorize;
use structopt::StructOpt;

use honey::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Honey",
    about = "A blazingly fast failure of a programming language."
)]
struct Opt {
    /// The input file to compile.
    #[structopt()]
    file: String,

    /// Prints a representation of all steps of the compilation (as specified by flags).
    #[structopt(short = "i", long = "info")]
    verbose: bool,

    /// Outputs the generated structures to a file.
    #[structopt(short = "s", long = "save")]
    save_structures: bool,

    /// Turns on lexical analysis.
    #[structopt(short = "l", long = "lex")]
    lex: bool,

    /// Turns on syntactic analysis (requires lexical analysis).
    #[structopt(short = "p", long = "parse")]
    parse: bool,

    /// Turns on semantic analysis (requires both lexical and syntactic).
    #[structopt(short = "v", long = "validate")]
    validate: bool,
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

    let tokens = if opt.lex { Some(lex(&source)) } else { None };

    let program = if tokens.is_some() && opt.parse {
        Some(parse(&tokens.clone().unwrap()))
    } else {
        None
    };

    let validator = if program.is_some() && opt.validate {
        Some(validate(&program.clone().unwrap()))
    } else {
        None
    };

    if opt.verbose {
        println!("\n{}\n\n{}", "File contents:".bold().yellow(), source,);

        if opt.lex {
            println!("------------\n");
            println!("{} {:#?}\n", "Tokens:".bold().green(), tokens);
        }

        if opt.parse {
            println!("------------\n");
            println!("{} {:#?}\n", "Parsed program:".bold().blue(), program);
        }

        if opt.validate {
            println!("\n------------");
            println!("{}: {:#?}\n", "Validator".bold().green(), validator);
        }
    }

    if opt.save_structures {
        std::fs::write("debug/generated/tokens", format!("{:#?}", &tokens))
            .expect("failed to write tokens to file");
        std::fs::write("debug/generated/parse_tree", format!("{:#?}", &program))
            .expect("failed to write parse tree to file");
        // std::fs::write("debug/generated/validator", format!("{:#?}", &validator))
        //     .expect("failed to write validator to file");
    }
}
