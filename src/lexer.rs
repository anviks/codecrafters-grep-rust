use crate::token::{Atom, Node, Repeat};

pub(crate) struct Lexer {
    pattern: Vec<char>,
    pos: usize,
    pub(crate) current_group: usize,
}

impl Lexer {
    pub(crate) fn new(pattern: &String) -> Self {
        Self {
            pattern: pattern.chars().collect(),
            pos: 0,
            current_group: 0,
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

    fn number(&mut self) -> usize {
        let mut s = String::new();
        while self.peek().is_ascii_digit() {
            s.push(self.consume());
        }
        s.parse::<usize>().unwrap()
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

    fn group(&mut self) -> Node {
        self.advance();

        let mut alternatives = vec![];

        self.current_group += 1;
        let index = self.current_group;

        while ![')', '\0'].contains(&self.peek()) {
            alternatives.push(self.analyze_until(&['|', ')', '\0']));
            if self.peek() == '|' {
                self.advance();
            }
        }

        self.advance();

        Node::new(Atom::Group {
            index,
            alternatives,
        })
    }

    fn analyze_until(&mut self, until: &[char]) -> Vec<Node> {
        let mut nodes = vec![];

        while !until.contains(&self.peek()) {
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
                '(' => nodes.push(self.group()),
                '\\' => {
                    self.advance();
                    match self.peek() {
                        'd' => {
                            nodes.push(Node::new(Atom::Digit));
                            self.advance();
                        }
                        'w' => {
                            nodes.push(Node::new(Atom::WordChar));
                            self.advance();
                        }
                        '0'..='9' => nodes.push(Node::new(Atom::BackReference(self.number()))),
                        _ => {}
                    }
                }
                '.' => {
                    nodes.push(Node::new(Atom::WildCard));
                    self.advance();
                }
                '+' => {
                    let node = nodes.last_mut().unwrap();
                    node.repeat = Repeat {
                        min: 1,
                        max: usize::MAX,
                    };
                    self.advance();
                }
                '?' => {
                    let node = nodes.last_mut().unwrap();
                    node.repeat = Repeat { min: 0, max: 1 };
                    self.advance();
                }
                '{' => {
                    let node = nodes.last_mut().unwrap();
                    self.advance();
                    let min = self.number();
                    let max = if self.consume() == ',' {
                        let num = if self.peek() == '}' {
                            usize::MAX
                        } else {
                            self.number()
                        };
                        self.advance();
                        num
                    } else {
                        min
                    };
                    node.repeat = Repeat { min, max };
                }
                '*' => {
                    let node = nodes.last_mut().unwrap();
                    node.repeat = Repeat {
                        min: 0,
                        max: usize::MAX,
                    };
                    self.advance();
                }
                _ => nodes.push(Node::new(Atom::Literal(self.consume()))),
            };
        }

        nodes
    }

    pub(crate) fn analyze(&mut self) -> Vec<Node> {
        let mut alternatives = vec![];
        while !self.is_end() {
            alternatives.push(self.analyze_until(&['|', '\0']));
            self.advance();
        }
        vec![Node::new(Atom::Group {
            alternatives,
            index: 0,
        })]
    }
}
