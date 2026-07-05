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
    process,
    slice::SliceIndex,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'E', long)]
    extended_regexp: bool,

    #[arg(short, long)]
    only_matching: bool,

    #[arg(long, default_value_t = ColorChoice::Never)]
    color: ColorChoice,

    pattern: String,

    filenames: Vec<String>,
}

fn slice<R>(chars: &[char], range: R) -> String
where
    R: SliceIndex<[char], Output = [char]>,
{
    chars[range].iter().collect()
}

fn print_matching_lines(
    matching_lines: &Vec<(Vec<char>, Vec<(usize, usize)>)>,
    only_matching: bool,
    show_color: bool,
    filename: Option<&String>,
) {
    let prefix = filename.map_or(String::new(), |s| s.to_owned() + ":");
    if only_matching {
        for (line, matches) in matching_lines {
            for (start, end) in matches {
                let text = slice(line, *start..*end);
                if show_color {
                    println!("\x1B[01;31m{}\x1B[m", text);
                } else {
                    println!("{}", text);
                }
            }
        }
    } else {
        for (line, matches) in matching_lines {
            print!("{}{}", prefix, slice(&line, ..matches[0].0));
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
}

fn main() {
    let args = Args::parse();
    // println!("{args:?}");

    if !args.extended_regexp {
        println!("Expected argument '-E' to be present");
        process::exit(1);
    }

    let mut lexer = Lexer::new(&args.pattern);
    let nodes = lexer.analyze();
    // println!("{nodes:#?}");

    let show_color = match args.color {
        ColorChoice::Auto if std::io::stdout().is_terminal() => true,
        ColorChoice::Always => true,
        _ => false,
    };

    let mut anything_matched = false;
    let mut inputs = vec![];

    if !args.filenames.is_empty() {
        for filename in &args.filenames {
            inputs.push(fs::read_to_string(filename).unwrap());
        }
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        inputs.push(input);
    }

    for (i, input) in inputs.iter().enumerate() {
        let matching_lines: Vec<(Vec<char>, Vec<(usize, usize)>)> = input
            .split('\n')
            .map(|line| {
                let chars = line.chars().collect();
                let matches = match_pattern(&chars, &nodes);
                (chars, matches)
            })
            .filter(|(_, matches)| !matches.is_empty())
            .collect();

        if matching_lines.len() > 0 {
            let filename = if args.filenames.len() > 1 {
                Some(&args.filenames[i])
            } else {
                None
            };
            print_matching_lines(&matching_lines, args.only_matching, show_color, filename);
            anything_matched = true;
        }
    }

    if anything_matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
