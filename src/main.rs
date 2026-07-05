#![allow(unused)]

mod lexer;
mod matcher;
mod token;

use crate::{
    lexer::Lexer,
    matcher::match_pattern,
    token::{Atom, Node, Repeat},
};
use clap::{ColorChoice, Parser};
use std::{
    env, fs,
    io::{self, IsTerminal, Read},
    ops::{Range, RangeBounds},
    process,
    slice::SliceIndex,
};

fn slice<R>(chars: &[char], range: R) -> String
where
    R: SliceIndex<[char], Output = [char]>,
{
    chars[range].iter().collect()
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'E', long)]
    extended_regexp: bool,

    #[arg(short, long)]
    only_matching: bool,

    #[arg(long, default_value_t = ColorChoice::Never)]
    color: ColorChoice,

    pattern: String,

    filename: Option<String>,
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    let args = Args::parse();

    if !args.extended_regexp {
        println!("Expected argument '-E' to be present");
        process::exit(1);
    }

    let mut lexer = Lexer::new(&args.pattern);
    let nodes = lexer.analyze();
    // println!("{:#?}", nodes);

    let mut input = String::new();
    if let Some(filename) = args.filename {
        input = fs::read_to_string(filename).unwrap();
    } else {
        io::stdin().read_to_string(&mut input).unwrap();
    }
    let matching_lines: Vec<(Vec<char>, Vec<(usize, usize)>)> = input
        .split('\n')
        .map(|line| {
            let chars = line.chars().collect();
            let matches = match_pattern(&chars, &nodes);
            (chars, matches)
        })
        .filter(|(_, matches)| !matches.is_empty())
        .collect();

    let show_color = match args.color {
        ColorChoice::Auto if std::io::stdout().is_terminal() => true,
        ColorChoice::Always => true,
        _ => false,
    };

    if matching_lines.len() > 0 {
        if args.only_matching {
            for (line, matches) in matching_lines {
                for (start, end) in matches {
                    let text = slice(&line, start..end);
                    if show_color {
                        println!("\x1B[01;31m{}\x1B[m", text);
                    } else {
                        println!("{}", text);
                    }
                }
            }
        } else {
            for (line, matches) in matching_lines {
                print!("{}", slice(&line, ..matches[0].0));
                let mut i = 0;
                while i < matches.len() {
                    let (start, end) = matches[i];
                    let text = slice(&line, start..end);
                    if show_color {
                        print!("\x1B[01;31m{}\x1B[m", text);
                    } else {
                        print!("{}", text);
                    }
                    i += 1;
                    if i < matches.len() {
                        print!("{}", slice(&line, end..matches[i].0));
                    } else {
                        println!("{}", slice(&line, end..));
                    }
                }
            }
        }
        process::exit(0)
    } else {
        process::exit(1)
    }
}
