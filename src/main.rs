use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::ExitCode;

use textstat::*;

fn main() -> ExitCode {
    let cli = Cli::parse();
    // if there is no arguments provided default to lines, words and bytes.
    let cli_options = if !cli.length && !cli.bytes && !cli.lines && !cli.characters && !cli.words {
        CliOptions {
            bytes: true,
            lines: true,
            words: true,
            ..CliOptions::default()
        }
    } else {
        CliOptions {
            length: cli.length,
            bytes: cli.bytes,
            lines: cli.lines,
            characters: cli.characters,
            words: cli.words,
        }
    };

    if run(cli, cli_options) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run(cli: Cli, cli_options: CliOptions) -> bool {
    let handles: Vec<_> = cli
        .file_paths
        .into_iter()
        .map(|path| {
            std::thread::spawn(move || {
                let result =
                    File::open(&path).and_then(|file| count(BufReader::new(file), &cli_options));
                (path, result)
            })
        })
        .collect();

    let mut ok = true;
    for handle in handles {
        match handle.join() {
            Ok((path, Ok(counts))) => print_counts(&path, &counts, &cli_options),
            Ok((path, Err(err))) => {
                eprintln!("{}: {err}", path.display());
                ok = false;
            }
            Err(_) => {
                eprintln!("textstat: worker thread panicked");
                ok = false;
            }
        }
    }
    ok
}

fn print_counts(path: &Path, counts: &Counts, opts: &CliOptions) {
    let mut output = format!("In the file {}. ", path.display());

    if opts.bytes {
        output.push_str(&format!("Bytes: {0} ", counts.bytes));
    }

    if opts.characters {
        output.push_str(&format!("Characters: {0} ", counts.characters));
    }

    if opts.words {
        output.push_str(&format!("Words: {0} ", counts.words));
    }

    if opts.length {
        output.push_str(&format!("Longest line: {0} ", counts.length));
    }

    if opts.lines {
        output.push_str(&format!("Lines: {0} ", counts.lines));
    }

    println!("{output}");
}
