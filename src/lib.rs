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

#[derive(Clone, Copy, Default)]
pub struct CliOptions {
    pub length: bool,
    pub bytes: bool,
    pub lines: bool,
    pub characters: bool,
    pub words: bool,
}

#[derive(Debug, Default, PartialEq)]
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

    while reader.read_line(&mut buffer)? != 0 {
        if cli.bytes {
            counts.bytes += count_bytes_str(&buffer);
        };

        if cli.characters {
            counts.characters += count_chars_str(&buffer);
        }

        if cli.length {
            let tmp_characters = count_chars_str(buffer.trim_end_matches(['\n', '\r']));

            if counts.length < tmp_characters {
                counts.length = tmp_characters;
            }
        }

        if cli.words {
            counts.words += count_words_str(&buffer);
        };

        if cli.lines {
            counts.lines += 1;
        };

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn count_bytes_correctly() {
        let ascii = String::from("Hello World");
        let cyrillic = String::from("Привет Мир");

        assert_eq!(11usize, count_bytes_str(&ascii));
        assert_eq!(19usize, count_bytes_str(&cyrillic));
    }

    #[test]
    fn count_chars_correctly() {
        let ascii = String::from("Hello World");
        let cyrillic = String::from("Привет Мир");

        assert_eq!(11usize, count_chars_str(&ascii));
        assert_eq!(10usize, count_chars_str(&cyrillic));
    }

    #[test]
    fn count_words_correctly() {
        let ascii = String::from("one two Three");
        let cyrillic = String::from("Раз Два три");

        assert_eq!(3usize, count_words_str(&ascii));
        assert_eq!(3usize, count_words_str(&cyrillic));
    }

    #[test]
    fn count_lines_correctly() {
        let ascii = String::from("One\nTwo\nThree\n\n");
        let cyrillic = String::from("Раз\nДва\nТри\n\n");
        let ascii_without_last_newline = String::from("one\ntwo\nthree");
        let opts = CliOptions {
            lines: true,
            ..CliOptions::default()
        };

        let buff = Cursor::new(ascii);
        assert_eq!(4usize, count(buff, &opts).unwrap().lines);
        let buff = Cursor::new(cyrillic);
        assert_eq!(4usize, count(buff, &opts).unwrap().lines);
        let buff = Cursor::new(ascii_without_last_newline);
        assert_eq!(3usize, count(buff, &opts).unwrap().lines);
    }

    #[test]
    fn count_length_of_longest_line() {
        let ascii = String::from("One\nTwo\nTwentyThree\n\n");
        let cyrillic = String::from("Раз\nДвадцатьДва\n\nТри\n");
        let opts = CliOptions {
            length: true,
            ..CliOptions::default()
        };

        let buff = Cursor::new(ascii);
        assert_eq!(11usize, count(buff, &opts).unwrap().length);
        let buff = Cursor::new(cyrillic);
        assert_eq!(11usize, count(buff, &opts).unwrap().length);
    }

    #[test]
    fn empty_request_returns_nothing() {
        let request = String::new();
        let buff = Cursor::new(request);
        let opts = CliOptions::default();

        let answer = count(buff, &opts).unwrap();

        let counts_reference = Counts::default();

        assert_eq!(counts_reference, answer);
    }
}
