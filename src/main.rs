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
    env,
    fs::{self, DirEntry, Metadata},
    io::{self, IsTerminal, Read},
    process,
    slice::SliceIndex,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'E', long)]
    extended_regexp: bool,

    #[arg(short, long)]
    only_matching: bool,

    #[arg(long, default_value_t = ColorChoice::Never)]
    color: ColorChoice,

    #[arg(short, long)]
    recursive: bool,

    pattern: String,

    paths: Vec<String>,
}

struct Input {
    string: String,
    filename: Option<String>,
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
    let mut had_error = false;
    let mut inputs = vec![];

    if !args.paths.is_empty() {
        let multiple_files = args.paths.len() > 1;
        for path in args.paths {
            match fs::metadata(&path) {
                Ok(meta) => {
                    if meta.is_file() {
                        inputs.push(Input {
                            string: fs::read_to_string(&path).unwrap(),
                            filename: if multiple_files { Some(path) } else { None },
                        });
                    } else if args.recursive {
                        for p in WalkDir::new(&path)
                            .into_iter()
                            .filter_map(Result::ok)
                            .filter(|e| e.file_type().is_file())
                            .map(|e| e.into_path())
                        {
                            inputs.push(Input {
                                string: fs::read_to_string(&p).unwrap(),
                                filename: Some(p.to_str().unwrap().to_string()),
                            });
                        }
                    } else {
                        eprintln!("grep: {}: Is a directory", &path);
                        had_error = true;
                    }
                }
                Err(e) => {
                    eprintln!("grep: {}: {}", &path, e);
                    had_error = true;
                }
            };
        }
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        inputs.push(Input {
            string: input,
            filename: None,
        });
    }

    for (i, input) in inputs.iter().enumerate() {
        let matching_lines: Vec<(Vec<char>, Vec<(usize, usize)>)> = input
            .string
            .split('\n')
            .map(|line| {
                let chars = line.chars().collect();
                let matches = match_pattern(&chars, &nodes);
                (chars, matches)
            })
            .filter(|(_, matches)| !matches.is_empty())
            .collect();

        if matching_lines.len() > 0 {
            print_matching_lines(
                &matching_lines,
                args.only_matching,
                show_color,
                input.filename.as_ref(),
            );
            anything_matched = true;
        }
    }

    if anything_matched && !had_error {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
