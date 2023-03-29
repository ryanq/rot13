use std::{io, ops::ControlFlow};

use anyhow::Result;
use clap::Parser;
use utf8_read::Reader;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(input) = cli.input.as_ref() {
        input
            .chars()
            .map(|c| if c.is_ascii_alphabetic() { rot13(c) } else { c })
            .for_each(|c| print!("{c}"));
        println!();
    } else {
        Reader::new(io::stdin().lock())
            .map(|c| c.map(|c| if c.is_ascii_alphabetic() { rot13(c) } else { c }))
            .try_for_each(|c| match c {
                Ok(c) => {
                    print!("{c}");
                    ControlFlow::Continue(())
                }
                Err(e) => {
                    eprintln!("\nError: {e}");
                    ControlFlow::Break(())
                }
            });
    }

    Ok(())
}

/// Transform text with the ROT-13 cipher.
///
/// The ROT-13 cipher is a weak cipher and should not be used to encode
/// sensitive data. It is best used to hide information from a casual glance
/// (e.g. spoilers, puzzle solutions, or offensive information).
#[derive(Debug, Parser)]
struct Cli {
    /// The text to transform. If no input is given, transform the data on stdin.
    input: Option<String>,
}

fn rot13(ch: char) -> char {
    match ch {
        'A' => 'N',
        'B' => 'O',
        'C' => 'P',
        'D' => 'Q',
        'E' => 'R',
        'F' => 'S',
        'G' => 'T',
        'H' => 'U',
        'I' => 'V',
        'J' => 'W',
        'K' => 'X',
        'L' => 'Y',
        'M' => 'Z',
        'N' => 'A',
        'O' => 'B',
        'P' => 'C',
        'Q' => 'D',
        'R' => 'E',
        'S' => 'F',
        'T' => 'G',
        'U' => 'H',
        'V' => 'I',
        'W' => 'J',
        'X' => 'K',
        'Y' => 'L',
        'Z' => 'M',
        'a' => 'n',
        'b' => 'o',
        'c' => 'p',
        'd' => 'q',
        'e' => 'r',
        'f' => 's',
        'g' => 't',
        'h' => 'u',
        'i' => 'v',
        'j' => 'w',
        'k' => 'x',
        'l' => 'y',
        'm' => 'z',
        'n' => 'a',
        'o' => 'b',
        'p' => 'c',
        'q' => 'd',
        'r' => 'e',
        's' => 'f',
        't' => 'g',
        'u' => 'h',
        'v' => 'i',
        'w' => 'j',
        'x' => 'k',
        'y' => 'l',
        'z' => 'm',
        _ => unimplemented!("ROT13 is only implemented for English alphabetic characters"),
    }
}
