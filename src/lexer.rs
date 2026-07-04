use crate::token::Token;

pub(crate) struct Lexer {
    pattern: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub(crate) fn new(pattern: &String) -> Self {
        Lexer {
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

    fn char_group(&mut self) -> Token {
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

        Token::CharGroup { chars, negated }
    }

    pub(crate) fn analyze(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.is_end() {
            match self.peek() {
                '^' => {
                    tokens.push(Token::Start);
                    self.advance();
                }
                '$' => {
                    tokens.push(Token::End);
                    self.advance();
                }
                '[' => tokens.push(self.char_group()),
                '\\' => {
                    self.advance();
                    match self.consume() {
                        'd' => tokens.push(Token::Digit),
                        'w' => tokens.push(Token::WordChar),
                        _ => {}
                    }
                }
                _ => tokens.push(Token::Literal(self.consume())),
            };
        }

        tokens
    }
}
