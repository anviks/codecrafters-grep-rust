use crate::token::{Atom, Node, Repeat};

pub(crate) struct Lexer {
    pattern: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub(crate) fn new(pattern: &String) -> Self {
        Self {
            pattern: pattern.chars().collect(),
            pos: 0,
        }
    }

    fn is_end(&self) -> bool {
        self.pos >= self.pattern.len()
    }

    fn peek(&self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.pattern[self.pos]
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn consume(&mut self) -> char {
        let char = self.peek();
        self.advance();
        char
    }

    fn char_group(&mut self) -> Node {
        self.advance();

        let negated = self.peek() == '^';
        if negated {
            self.advance();
        }

        let mut chars = vec![];
        while !self.is_end() && self.peek() != ']' {
            chars.push(self.consume());
        }

        self.advance();

        Node::new(Atom::CharGroup { chars, negated })
    }

    pub(crate) fn analyze(&mut self) -> Vec<Node> {
        let mut nodes = vec![];

        while !self.is_end() {
            match self.peek() {
                '^' => {
                    nodes.push(Node::new(Atom::Start));
                    self.advance();
                }
                '$' => {
                    nodes.push(Node::new(Atom::End));
                    self.advance();
                }
                '[' => nodes.push(self.char_group()),
                '\\' => {
                    self.advance();
                    match self.consume() {
                        'd' => nodes.push(Node::new(Atom::Digit)),
                        'w' => nodes.push(Node::new(Atom::WordChar)),
                        _ => {}
                    }
                }
                '+' => {
                    let mut node = nodes.last_mut().unwrap();
                    node.repeat = Repeat { min: 1, max: None };
                    self.advance();
                }
                '?' => {
                    let mut node = nodes.last_mut().unwrap();
                    node.repeat = Repeat {
                        min: 0,
                        max: Some(1),
                    };
                    self.advance();
                }
                '*' => {
                    let mut node = nodes.last_mut().unwrap();
                    node.repeat = Repeat { min: 0, max: None };
                    self.advance();
                }
                _ => nodes.push(Node::new(Atom::Literal(self.consume()))),
            };
        }

        nodes
    }
}
