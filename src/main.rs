#![allow(unused)]

mod lexer;
mod token;

use std::env;
use std::io;
use std::process;

use crate::lexer::Lexer;
use crate::token::Token;

fn match_pattern(input_line: &str, pattern: &mut Vec<Token>) -> bool {
    let mut start = 0;
    let mut chars: Vec<char> = input_line.chars().collect();

    let start_anchor = if let Some(Token::Start) = pattern.first() {
        pattern.remove(0);
        true
    } else {
        false
    };

    let end_anchor = if let Some(Token::End) = pattern.last() {
        pattern.pop();
        true
    } else {
        false
    };

    'outer: while start < chars.len() {
        let mut i = start;

        for tok in &mut *pattern {
            if i >= chars.len() {
                start += 1;
                continue 'outer;
            }
            let char = chars[i];

            let matches = match tok {
                Token::Start | Token::End => false,
                Token::Digit => {
                    i += 1;
                    char.is_digit(10)
                }
                Token::Literal(c) => {
                    i += 1;
                    char == *c
                }
                Token::WordChar => {
                    i += 1;
                    char.is_ascii_alphanumeric() || char == '_'
                }
                Token::CharGroup { chars, negated } => {
                    i += 1;
                    !*negated && chars.contains(&char) || *negated && !chars.contains(&char)
                }
                Token::CapturingGroup => todo!(),
            };

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
    let mut tokens = lexer.analyze();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &mut tokens) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
