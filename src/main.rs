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

fn match_repeat(
    atom: &Atom,
    repeat: &Repeat,
    rest: &[Node],
    text: &[char],
    pos: usize,
) -> Option<usize> {
    let mut matched: usize = 0;
    while repeat.max.map(|m| matched < m as usize).unwrap_or(true)
        && pos + matched < text.len()
        && atom.matches(text[pos + matched])
    {
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

fn match_pattern(input_line: &str, pattern: &Vec<Node>) -> bool {
    let chars: Vec<char> = input_line.chars().collect();

    let mut start = 0;
    while start <= chars.len() {
        let matches = match_here(pattern, &chars, start);
        if let Some(n) = matches {
            return true;
        }
        start += 1;
    }

    false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut lexer = Lexer::new(&pattern);
    let nodes = lexer.analyze();
    println!("{:#?}", nodes);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<&str> = input
        .split('\n')
        .filter(|line| match_pattern(line, &nodes))
        .collect();

    if lines.len() > 0 {
        println!("{}", lines.join("\n"));
        process::exit(0)
    } else {
        process::exit(1)
    }
}
