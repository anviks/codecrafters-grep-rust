#![allow(unused)]

mod lexer;
mod token;

use crate::{
    lexer::Lexer,
    token::{Atom, Node, Repeat},
};
use std::{
    env,
    io::{self, Read},
    process,
};
use clap::Parser;

fn match_repeat(
    atom: &Atom,
    repeat: &Repeat,
    rest: &[Node],
    text: &[char],
    pos: usize,
) -> Option<usize> {
    let mut matched: usize = 0;
    while matched < repeat.max && pos + matched < text.len() && atom.matches(text[pos + matched]) {
        matched += 1;
    }

    if matched < repeat.min as usize {
        return None;
    }

    loop {
        if let Some(n) = match_here(rest, text, pos + matched) {
            break Some(n);
        }
        if matched == repeat.min as usize {
            break None;
        }
        matched -= 1;
    }
}

fn match_here(nodes: &[Node], text: &[char], pos: usize) -> Option<usize> {
    let Some((node, rest)) = nodes.split_first() else {
        return Some(pos);
    };

    match &node.atom {
        Atom::Start => {
            if pos == 0 {
                match_here(rest, text, pos)
            } else {
                None
            }
        }
        Atom::End => {
            if pos == text.len() {
                match_here(rest, text, pos)
            } else {
                None
            }
        }
        Atom::Group { alternatives } => {
            for alt in alternatives {
                if let Some(p) = match_here(alt, text, pos)
                    && let Some(p_rest) = match_here(rest, text, p)
                {
                    return Some(p_rest);
                }
            }

            None
        }
        atom => match_repeat(atom, &node.repeat, rest, text, pos),
    }
}

fn match_pattern(input_line: &str, pattern: &Vec<Node>) -> Vec<(usize, usize)> {
    let chars: Vec<char> = input_line.chars().collect();
    let mut results = vec![];

    let mut start = 0;
    while start <= chars.len() {
        let matches = match_here(pattern, &chars, start);
        if let Some(n) = matches {
            results.push((start, n));
            start = n;
        }
        start += 1;
    }

    results
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'E', long)]
    extended_regexp: bool,

    #[arg(short, long)]
    only_matching: bool,

    pattern: String,
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
    io::stdin().read_to_string(&mut input).unwrap();
    let matching_lines: Vec<(&str, Vec<(usize, usize)>)> = input
        .split('\n')
        .map(|line| (line, match_pattern(line, &nodes)))
        .filter(|(_, matches)| matches.len() > 0)
        .collect();

    if matching_lines.len() > 0 {
        if args.only_matching {
            for (line, matches) in matching_lines {
                for (start, end) in matches {
                    println!("{}", &line[start..end])
                }
            }
        } else {
            for (line, _) in matching_lines {
                println!("{}", line);
            }
        }
        process::exit(0)
    } else {
        process::exit(1)
    }
}
