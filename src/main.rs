use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern == "\\d" {
        for c in input_line.chars() {
            if c.is_digit(10) {
                return true;
            }
        }
        false
    } else if pattern == "\\w" {
        for c in input_line.chars() {
            if c.is_ascii_alphanumeric() || c == '_' {
                return true;
            }
        }
        false
    } else if pattern.starts_with("[") {
        let chars: Vec<char> = pattern.chars().skip(1).collect();
        let inverse = if let Some('^') = chars.get(0) {
            true
        } else {
            false
        };
        for char in chars {
            if char == ']' {
                break;
            }
            if !inverse && input_line.contains(char) || inverse && !input_line.contains(char) {
                return true;
            }
        }
        false
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
