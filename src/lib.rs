use clap::Parser;
use std::io::BufRead;
use std::path::PathBuf;

/// Count lines, words, bytes and characters in files.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Print the length of the longest line.
    #[arg(short = 'L', long)]
    pub length: bool,

    /// Print the number of bytes.
    #[arg(short = 'c', long)]
    pub bytes: bool,

    /// Print the number of lines.
    #[arg(short = 'l', long)]
    pub lines: bool,

    /// Print the number of characters.
    #[arg(short = 'm', long)]
    pub characters: bool,

    /// Print the number of words.
    #[arg(short = 'w', long)]
    pub words: bool,

    /// Input files.
    pub file_paths: Vec<PathBuf>,
}

#[derive(Clone, Copy)]
pub struct CliOptions {
    pub length: bool,
    pub bytes: bool,
    pub lines: bool,
    pub characters: bool,
    pub words: bool,
}

#[derive(Debug, Default)]
pub struct Counts {
    pub length: usize,
    pub bytes: usize,
    pub lines: usize,
    pub characters: usize,
    pub words: usize,
}

pub fn count<R: BufRead>(mut reader: R, cli: &CliOptions) -> std::io::Result<Counts> {
    let mut counts = Counts::default();
    let mut buffer = String::new();
    let mut temp_count_bytes;

    while reader.read_line(&mut buffer)? != 0 {
        if cli.bytes || cli.length {
            temp_count_bytes = count_bytes_str(&buffer);
            // count bytes
            if cli.bytes {
                counts.bytes += temp_count_bytes;
            };
            // update length of biggest line
            if cli.length && counts.length < temp_count_bytes {
                counts.length = temp_count_bytes;
            }
        }
        // count chars
        if cli.characters {
            counts.characters += count_chars_str(&buffer);
        };
        // count words
        if cli.words {
            counts.words += count_words_str(&buffer);
        };
        // count ammount of lines
        if cli.lines {
            counts.lines += 1;
        };

        // clear buffer before end of the loop
        buffer.clear();
    }

    Ok(counts)
}

fn count_bytes_str(str: &str) -> usize {
    str.len()
}

fn count_chars_str(str: &str) -> usize {
    str.chars().count()
}

fn count_words_str(str: &str) -> usize {
    str.split_ascii_whitespace().count()
}
