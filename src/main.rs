#![allow(unused)]

mod lexer;
mod token;

use crate::{
    lexer::Lexer,
    token::{Atom, Node},
};
use std::{env, io, process};

fn match_pattern(input_line: &str, pattern: &mut Vec<Node>) -> bool {
    let mut start = 0;
    let mut chars: Vec<char> = input_line.chars().collect();

    let start_anchor = if let Some(Node {
        atom: Atom::Start,
        repeat,
    }) = pattern.first()
    {
        pattern.remove(0);
        true
    } else {
        false
    };

    let end_anchor = if let Some(Node {
        atom: Atom::End,
        repeat,
    }) = pattern.last()
    {
        pattern.pop();
        true
    } else {
        false
    };

    'outer: while start < chars.len() {
        let mut i = start;

        for node in &mut *pattern {
            if i >= chars.len() {
                start += 1;
                continue 'outer;
            }
            let char = chars[i];
            let matches = node.atom.matches(char);
            i += 1;

            if !matches {
                if start_anchor {
                    return false;
                }
                start += 1;
                continue 'outer;
            }
        }

        if end_anchor && i != chars.len() {
            start += 1;
            continue;
        }

        return true;
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
    let mut nodes = lexer.analyze();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &mut nodes) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
