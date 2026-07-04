#![allow(unused)]

mod lexer;
mod token;

use std::env;
use std::io;
use std::process;

use crate::lexer::Lexer;
use crate::token::Token;

fn match_pattern(input_line: &str, pattern: &Vec<Token>) -> bool {
    let mut start = 0;
    let chars: Vec<char> = input_line.chars().collect();

    'outer: while start < chars.len() {
        let mut i = start;

        for tok in pattern {
            let char = chars[i];

            let matches = match tok {
                Token::Start | Token::End => false,
                Token::Digit => {
                    i += 1;
                    char.is_digit(10)
                }
                Token::Literal(s) => {
                    let m = i + s.len() <= chars.len()
                        && s.chars().eq(chars[i..i + s.len()].iter().copied());
                    i += s.len();
                    m
                }
                Token::WordChar => {
                    i += 1;
                    char.is_ascii_alphanumeric() || char == '_'
                }
                Token::CharGroup { chars, negated } => {
                    i += 1;
                    !negated && chars.contains(&char) || *negated && !chars.contains(&char)
                }
                Token::CapturingGroup => todo!(),
            };

            if !matches {
                start += 1;
                continue 'outer;
            }
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
    let tokens = lexer.analyze();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &tokens) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
