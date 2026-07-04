#![allow(unused)]

mod lexer;
mod token;

use crate::{
    lexer::Lexer,
    token::{Atom, Node, Repeat},
};
use std::{env, io, process};

fn match_repeat(atom: &Atom, repeat: &Repeat, rest: &[Node], text: &[char], pos: usize) -> bool {
    let mut matched: usize = 0;
    while repeat.max.map(|m| matched < m as usize).unwrap_or(true)
        && pos + matched < text.len()
        && atom.matches(text[pos + matched])
    {
        matched += 1;
    }

    while matched >= repeat.min as usize {
        if match_here(rest, text, pos + matched) {
            return true;
        }
        matched -= 1;
    }

    false
}

fn match_here(nodes: &[Node], text: &[char], pos: usize) -> bool {
    let Some((node, rest)) = nodes.split_first() else {
        return true;
    };

    match &node.atom {
        Atom::Start => pos == 0 && match_here(rest, text, pos),
        Atom::End => pos == text.len() && match_here(rest, text, pos),
        atom => match_repeat(atom, &node.repeat, rest, text, pos),
    }
}

fn match_pattern(input_line: &str, pattern: &Vec<Node>) -> bool {
    let chars: Vec<char> = input_line.chars().collect();

    let mut start = 0;
    while start <= chars.len() {
        let matches = match_here(pattern, &chars, start);
        if matches {
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
    // println!("{:#?}", nodes);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &nodes) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
