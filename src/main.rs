use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process;

use wc_rs::*;

fn main() {
    let cli = Cli::parse();
    let cli_options = CliOptions {
        length: cli.length,
        bytes: cli.bytes,
        lines: cli.lines,
        characters: cli.characters,
        words: cli.words,
    };

    run(cli, cli_options).unwrap_or_else(|err| {
        eprintln!("Problem during execution: {err}");
        process::exit(1);
    })
}

fn run(cli: Cli, cli_options: CliOptions) -> Result<(), Box<dyn Error>> {
    // open file path into reader
    for file in cli.file_paths {
        let file_name = format!("{}", file.display());
        let file_hand = File::open(file)?;
        let reader = BufReader::new(file_hand);

        let count = count(reader, &cli_options)?;
        print!("In the file {file_name}. ");
        if cli_options.bytes {
            print!("Bytes: {0} ", count.bytes);
        }
        if cli_options.characters {
            print!("Characters: {0} ", count.characters);
        }
        if cli_options.words {
            print!("Words: {0} ", count.words);
        }
        if cli_options.length {
            print!("Longest line: {0} ", count.length);
        }
        if cli_options.lines {
            print!("Lines: {0} ", count.lines);
        }
        println!();
    }
    Ok(())
}
