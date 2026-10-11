# textstat

A small command-line tool, written in Rust, that counts lines, words, bytes and
characters in text files and finds the length of the longest line. It is
similar to `wc`, but it is a learning project, not a drop-in replacement. See
[Differences from `wc`](#differences-from-wc).

## Build and install

Requires Rust 1.85 or newer (the crate uses edition 2024).

```sh
git clone https://github.com/pixelcmnd/textstat.git
cd textstat
cargo build --release     # binary: target/release/textstat
cargo install --path .    # or install it into ~/.cargo/bin
```

## Usage

```
textstat [OPTIONS] [FILE_PATHS]...
```

| Option | Long form      | Prints                     |
| ------ | -------------- | -------------------------- |
| `-l`   | `--lines`      | number of lines            |
| `-w`   | `--words`      | number of words            |
| `-c`   | `--bytes`      | number of bytes            |
| `-m`   | `--characters` | number of characters       |
| `-L`   | `--length`     | length of the longest line |

With no options, `textstat` prints bytes, words and lines. Short options can be
combined (`-lw`).

```console
$ textstat notes.txt
In the file notes.txt. Bytes: 44 Words: 9 Lines: 3

$ textstat -lw notes.txt utf-8.txt
In the file notes.txt. Words: 9 Lines: 3
In the file utf-8.txt. Words: 2 Lines: 1

$ textstat -Lmc utf-8.txt
In the file utf-8.txt. Bytes: 20 Characters: 11 Longest line: 10
```

Files are counted in parallel, one thread per file, and results are printed in
the order the files were given. Fields always appear in the same order (bytes,
characters, words, longest line, lines), regardless of the order of the options.
If a file cannot be read, an error naming that file is printed to stderr,
the remaining files are still processed, and the exit status is 1.

```console
$ textstat -l notes.txt missing.txt
In the file notes.txt. Lines: 3
missing.txt: No such file or directory (os error 2)
```

## Differences from `wc`

- **Output format.** The output is a labelled sentence per file, not `wc`'s
  columns. There is no `total` line.
- **Last line without a newline.** `wc -l` counts newline characters, so a file
  that does not end with `\n` is counted one line short. `textstat` counts
  that last line. For `one\ntwo\nthree` (no final newline), `textstat -l`
  prints 3 and `wc -l` prints 2.
- **Words** are separated by ASCII whitespace only. Unicode whitespace such as
  the non-breaking space does not separate words.
- **Input must be valid UTF-8.** A file that is not valid UTF-8 produces an
  error instead of counts.
- **No standard input.** Files must be given as arguments. With no files,
  `textstat` prints nothing and exits with status 0.
- **`-m` and `-c`** count everything, including line terminators, as `wc` does.
- **`-L`** counts characters, not bytes or display width, and does not include
  the trailing `\n` or `\r\n`. A tab counts as one character. `wc -L` behaves
  differently between implementations, so results can differ.

## License

MIT. See [LICENSE](LICENSE).
